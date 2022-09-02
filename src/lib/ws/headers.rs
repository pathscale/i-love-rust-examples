use eyre::*;

use crate::ws::Connection;
use async_tungstenite::tungstenite::handshake::server::{
    Callback, ErrorResponse, Request, Response,
};
use futures::future::BoxFuture;
use futures::FutureExt;
use std::collections::HashMap;
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
    fn auth(&self, header: String, conn: Connection) -> BoxFuture<'static, Result<Connection>>;
}
pub struct SimpleAuthContoller;
impl AuthController for SimpleAuthContoller {
    // 0login, 1pepe_pablo, 2AHJQ6X1H68SK8D9P6WW0, 31, 424787297130491616, 5android
    fn auth(&self, _header: String, conn: Connection) -> BoxFuture<'static, Result<Connection>> {
        async move { Ok(conn) }.boxed()
    }
}
pub struct ComplexAuthContoller;

// TODO: consider: should we convert all methods to req/resp?
impl AuthController for ComplexAuthContoller {
    // 0login, 1pepe_pablo, 2AHJQ6X1H68SK8D9P6WW0, 31, 424787297130491616, 5android
    fn auth(&self, header: String, mut conn: Connection) -> BoxFuture<'static, Result<Connection>> {
        async move {
            let splits = header
                .split(",")
                .map(|x| x.trim())
                .map(|x| (&x[..1], &x[1..]))
                .collect::<HashMap<&str, &str>>();
            let method = splits
                .get("0")
                .ok_or_else(|| eyre!("Could not find method"))?;
            if *method == "login" {
                let username = splits
                    .get("1")
                    .ok_or_else(|| eyre!("Could not find username"))?;
                let password_hash = splits
                    .get("2")
                    .ok_or_else(|| eyre!("Could not find password"))?;
                // TODO: what does it mean?
                let field_3 = splits
                    .get("3")
                    .ok_or_else(|| eyre!("Could not find field_3"))?;

                // TODO: what does it mean?
                let field_4 = splits
                    .get("4")
                    .ok_or_else(|| eyre!("Could not find field_4"))?;

                let device = splits
                    .get("5")
                    .ok_or_else(|| eyre!("Could not find device"))?;
                info!(
                    "Logging in: {} {} {} {} {}",
                    username, password_hash, field_3, field_4, device
                );
                conn.user_id = 1; // TODO: find user_id from database
                conn.role = 1;

                Ok(conn)
            } else {
                bail!("Could not process method {}", method)
            }
        }
        .boxed()
    }
}
