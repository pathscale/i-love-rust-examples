

use eyre::*;



use tracing::*;

use async_tungstenite::tungstenite::handshake::server::{Callback, Request, Response, ErrorResponse};



pub struct VerifyProtocol {
    pub tx: tokio::sync::mpsc::Sender<String>,
}

impl Callback for VerifyProtocol {
    fn on_request(self, request: &Request, response: Response) -> Result<Response, ErrorResponse> {
        debug!("on_request: {:?}", request);
        let protocol = request.headers().get("Sec-WebSocket-Protocol");
        println!("Sec-WebSocket-Protocol: {:?}", protocol);
        self.tx.send(match protocol {
            None => { "".to_string() }
            Some(x) => { x.to_str().unwrap_or("invalid utf-8 string").to_string() }
        });
        Ok(response)
    }
}

