use crate::database::SimpleDbClient;
use async_compat::{Compat, CompatExt};
use async_tungstenite::tungstenite::error::ProtocolError;
use async_tungstenite::tungstenite::http::StatusCode;
use async_tungstenite::tungstenite::Error as WsError;
use async_tungstenite::tungstenite::Message;
use async_tungstenite::WebSocketStream;
use dashmap::DashMap;
use eyre::*;
use futures::stream::{SplitSink, SplitStream};
use futures::SinkExt;
use futures::StreamExt;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::*;

use crate::toolbox::{RequestContext, Toolbox};
use crate::utils::get_log_id;
use crate::ws::basics::{Connection, RequestHandlerErased, WsRequest};
use crate::ws::{
    request_error_to_resp, AuthController, SimpleAuthContoller, VerifyProtocol, WsEndpoint,
    WsResponse,
};
use model::endpoint::EndpointSchema;
use tokio::net::TcpStream;

pub struct WsStream {
    ws_sink: SplitSink<WebSocketStream<Compat<TcpStream>>, Message>,
    conn: Arc<Connection>,
}

pub struct WsMessage {
    pub connection_id: u32,
    pub message: WsResponse,
}
pub struct WebsocketServer {
    pub auth_controller: Arc<dyn AuthController>,
    pub handlers: HashMap<u32, WsEndpoint>,
    pub connection: DashMap<u32, WsStream>,
    pub message_receiver: Option<mpsc::Receiver<WsMessage>>,
    pub toolbox: Toolbox,
}
impl Default for WebsocketServer {
    fn default() -> Self {
        let (msg_tx, msg_rx) = mpsc::channel(100);

        Self {
            auth_controller: Arc::new(SimpleAuthContoller),
            handlers: Default::default(),
            connection: Default::default(),
            message_receiver: Some(msg_rx),
            toolbox: Toolbox::new(msg_tx),
        }
    }
}
fn wrap_ws_error<T>(err: Result<T, WsError>) -> Result<T> {
    err.map_err(|x| eyre!(x))
}

impl WebsocketServer {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn add_database(&mut self, db: SimpleDbClient) {
        self.toolbox.set_db(db);
    }
    pub fn add_handler(
        &mut self,
        schema: EndpointSchema,
        handler: impl RequestHandlerErased + 'static,
    ) {
        self.add_handler_erased(schema, Arc::new(handler))
    }
    pub fn add_handler_erased(
        &mut self,
        schema: EndpointSchema,
        handler: Arc<dyn RequestHandlerErased>,
    ) {
        let old = self
            .handlers
            .insert(schema.code, WsEndpoint { schema, handler });
        if let Some(old) = old {
            panic!(
                "Overwriting handler for endpoint {} {}",
                old.schema.code, old.schema.name
            );
        }
    }
    async fn handle_request(self: Arc<Self>, addr: SocketAddr, stream: TcpStream) {
        let result: Result<()> = async {
            let (tx, mut rx) = mpsc::channel(1);
            let stream = wrap_ws_error(
                async_tungstenite::accept_hdr_async(stream.compat(), VerifyProtocol { tx }).await,
            )?;
            let mut conn = Connection {
                connection_id: 0,
                user_id: 0,
                role: 0,
                address: addr.ip(),
            };
            let headers = rx
                .recv()
                .await
                .ok_or_else(|| eyre!("Failed to receive ws headers"))?;
            conn = self.auth_controller.auth(headers, conn).await?;

            let (ws_sink, ws_stream) = stream.split();

            let stream = WsStream {
                ws_sink: ws_sink,
                conn: Arc::new(conn),
            };
            let conn = Arc::clone(&stream.conn);
            self.connection.insert(conn.connection_id, stream);
            tokio::spawn(Arc::clone(&self).recv_msg(conn, ws_stream));
            Ok(())
        }
        .await;
        if let Err(err) = result {
            error!(?addr, "Error while processing {:?}", err)
        }
    }
    pub async fn recv_msg(
        self: Arc<Self>,
        conn: Arc<Connection>,
        mut reader: SplitStream<WebSocketStream<Compat<TcpStream>>>,
    ) {
        let addr = conn.address;
        let context = RequestContext {
            connection_id: conn.connection_id,
            user_id: conn.user_id,
            seq: 0,
            method: 0,
            log_id: get_log_id(),
        };
        while let Some(msg) = reader.next().await {
            match msg {
                Ok(req) => {
                    let obj: Result<WsRequest, _> = match req {
                        Message::Text(t) => {
                            debug!(?addr, "Handling request {}", t);

                            serde_json::from_str(&t)
                        }
                        Message::Binary(b) => {
                            debug!(?addr, "Handling request <BIN>");
                            serde_json::from_slice(&b)
                        }
                        Message::Ping(_) => {
                            continue;
                        }
                        Message::Pong(_) => {
                            continue;
                        }
                        Message::Close(_) => {
                            info!(?addr, "Receive side terminated");
                            break;
                        }
                    };
                    let req = match obj {
                        Ok(req) => req,
                        Err(err) => {
                            self.toolbox.send(
                                &context,
                                request_error_to_resp(
                                    &context,
                                    StatusCode::BAD_REQUEST.into(),
                                    err,
                                ),
                            );
                            continue;
                        }
                    };
                    let context = RequestContext {
                        seq: req.seq,
                        method: req.method,
                        ..context
                    };
                    let handler = self.handlers.get(&req.method);
                    let handler = match handler {
                        Some(handler) => handler,
                        None => {
                            self.toolbox.send(
                                &context,
                                request_error_to_resp(
                                    &context,
                                    StatusCode::NOT_FOUND.into(),
                                    eyre!("Could not find handler for method code {}", req.method),
                                ),
                            );
                            continue;
                        }
                    };
                    handler
                        .handler
                        .handle(&self.toolbox, context, Arc::clone(&conn), req);
                }
                Err(WsError::Protocol(ProtocolError::ResetWithoutClosingHandshake)) => {
                    info!(?addr, "Receive side terminated");
                    break;
                }
                Err(err) => {
                    error!(?addr, "Error while receiving {:?}", err);
                    break;
                }
            }
        }
        self.connection.remove(&context.connection_id);
        info!(?addr, "Connection closed");
    }
    pub async fn send_msg(self: Arc<Self>, mut message_receiver: mpsc::Receiver<WsMessage>) {
        while let Some(msg) = message_receiver.recv().await {
            let conn = self.connection.get_mut(&msg.connection_id);
            if let Some(mut conn) = conn {
                let self1 = &msg.message;
                let result = conn
                    .ws_sink
                    .send(Message::Text(
                        serde_json::to_string(self1).expect("Failed to dump json(impossible)"),
                    ))
                    .await;
                if let Err(err) = result {
                    error!(?conn.conn.address, "Error while sending {:?}", err);
                }
            } else {
                error!(?msg.connection_id, "Connection not found");
            }
        }
    }
    pub async fn listen(mut self, addr: &str) -> Result<()> {
        info!("Listening on {}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let message_receiver = self.message_receiver.take().unwrap();
        let this = Arc::new(self);
        tokio::spawn(Arc::clone(&this).send_msg(message_receiver));
        loop {
            let (stream, addr) = listener.accept().await?;

            info!("Accepted stream from {}", addr);
            tokio::spawn(Arc::clone(&this).handle_request(addr, stream));
        }
    }
}
