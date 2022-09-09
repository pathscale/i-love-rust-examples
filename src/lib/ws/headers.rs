use eyre::*;

use crate::ws::Connection;
use futures::future::BoxFuture;
use futures::FutureExt;
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
    fn auth(&self, header: String, conn: Connection) -> BoxFuture<'static, Result<Connection>>;
}
pub struct SimpleAuthContoller;
impl AuthController for SimpleAuthContoller {
    // 0login, 1pepe_pablo, 2AHJQ6X1H68SK8D9P6WW0, 31, 424787297130491616, 5android
    fn auth(&self, _header: String, conn: Connection) -> BoxFuture<'static, Result<Connection>> {
        async move { Ok(conn) }.boxed()
    }
}
