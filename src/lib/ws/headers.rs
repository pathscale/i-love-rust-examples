use eyre::*;

use crate::toolbox::{RequestContext, Toolbox};
use crate::ws::{Connection, RequestHandlerErased, WsEndpoint, WsRequest};
use convert_case::Case;
use convert_case::Casing;
use dashmap::DashMap;
use futures::future::BoxFuture;
use futures::FutureExt;
use model::endpoint::*;
use model::types::Type;
use std::collections::HashMap;
use std::sync::Arc;
use tokio_tungstenite::tungstenite::handshake::server::{
    Callback, ErrorResponse, Request, Response,
};
use tracing::*;

pub struct VerifyProtocol {
    pub tx: tokio::sync::mpsc::Sender<String>,
}

impl Callback for VerifyProtocol {
    fn on_request(self, request: &Request, response: Response) -> Result<Response, ErrorResponse> {
        debug!("VerifyProtocol: {:?}", request);
        let protocol = request
            .headers()
            .get("Sec-WebSocket-Protocol")
            .or_else(|| request.headers().get("sec-websocket-protocol"));

        self.tx
            .try_send(match protocol {
                Some(protocol) => protocol
                    .to_str()
                    .map_err(|_| {
                        ErrorResponse::new(Some(
                            "Sec-WebSocket-Protocol is not valid utf-8".to_owned(),
                        ))
                    })?
                    .to_string(),
                None => "".to_string(),
            })
            .unwrap();
        Ok(response)
    }
}
pub trait AuthController: Sync + Send {
    fn auth(&self, header: String, conn: Arc<Connection>) -> BoxFuture<'static, Result<()>>;
}
pub struct SimpleAuthContoller;
impl AuthController for SimpleAuthContoller {
    fn auth(&self, _header: String, _conn: Arc<Connection>) -> BoxFuture<'static, Result<()>> {
        async move { Ok(()) }.boxed()
    }
}

pub struct EndpointAuthController {
    pub auth_endpoints: Arc<DashMap<String, WsEndpoint>>,
    pub toolbox: Toolbox,
}
impl EndpointAuthController {
    pub fn new(toolbox: Toolbox) -> Self {
        Self {
            auth_endpoints: Default::default(),
            toolbox,
        }
    }
    pub fn add_auth_endpoint(
        &self,
        schema: EndpointSchema,
        handler: impl RequestHandlerErased + 'static,
    ) {
        self.auth_endpoints.insert(
            schema.name.to_ascii_lowercase(),
            WsEndpoint {
                schema,
                handler: Arc::new(handler),
            },
        );
    }
}

impl AuthController for EndpointAuthController {
    fn auth(&self, header: String, conn: Arc<Connection>) -> BoxFuture<'static, Result<()>> {
        let mut toolbox = self.toolbox.clone();

        let endpoints = self.auth_endpoints.clone();
        async move {
            let splits = header
                .split(",")
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(|x| (&x[..1], &x[1..]))
                .collect::<HashMap<&str, &str>>();

            let method = splits.get("0").context("Could not find method")?;
            let endpoint = endpoints
                .get(*method)
                .with_context(|| format!("Could not find endpoint for method {}", method))?;
            let mut params = serde_json::Map::new();
            for (index, param) in endpoint.schema.parameters.iter().enumerate() {
                let index = index + 1;
                match splits.get(&index.to_string().as_str()) {
                    Some(value) => {
                        params.insert(
                            param.name.to_case(Case::Camel),
                            match param.ty {
                                Type::String => serde_json::Value::String(value.to_string()),
                                Type::Int => serde_json::Value::Number(
                                    value
                                        .parse::<i64>()
                                        .with_context(|| {
                                            format!("Failed to parse integer: {}", value)
                                        })?
                                        .into(),
                                ),
                                Type::Boolean => serde_json::Value::Bool(
                                    value
                                        .parse::<bool>()
                                        .with_context(|| {
                                            format!("Failed to parse boolean: {}", value)
                                        })?
                                        .into(),
                                ),
                                _ => todo!("Implement other types"),
                            },
                        );
                    }
                    None => {
                        bail!("Could not find param {} {}", param.name, index);
                    }
                }
            }

            let tasks = toolbox.collect_tasks(|toolbox| {
                endpoint.handler.handle(
                    &toolbox,
                    RequestContext {
                        connection_id: conn.connection_id,
                        user_id: 0,
                        seq: 0,
                        method: 0,
                        log_id: conn.log_id,
                    },
                    conn,
                    WsRequest {
                        method: endpoint.schema.code,
                        seq: 0,
                        params: serde_json::Value::Object(params),
                    },
                )
            });
            for t in tasks {
                t.await?;
            }
            Ok(())
        }
        .boxed()
    }
}
