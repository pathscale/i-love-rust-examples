use eyre::*;

use tracing::*;

use async_tungstenite::tungstenite::handshake::server::{
    Callback, ErrorResponse, Request, Response,
};

pub struct VerifyProtocol {
    pub tx: tokio::sync::mpsc::Sender<String>,
}

impl Callback for VerifyProtocol {
    fn on_request(self, request: &Request, response: Response) -> Result<Response, ErrorResponse> {
        debug!("on_request: {:?}", request);
        let protocol = request
            .headers()
            .get("Sec-WebSocket-Protocol")
            .ok_or_else(|| ErrorResponse::new(Some("No Sec-WebSocket-Protocol".to_owned())))?;
        self.tx
            .try_send(
                protocol
                    .to_str()
                    .map_err(|_| {
                        ErrorResponse::new(Some(
                            "Sec-WebSocket-Protocol is not valid utf-8".to_owned(),
                        ))
                    })?
                    .to_string(),
            )
            .unwrap();
        Ok(response)
    }
}
