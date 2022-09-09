use crate::error_code::ErrorCode;
use crate::log::LogLevel;
use crate::ws::{WsLogResponse, WsRequestGeneric, WsResponse, WsResponseGeneric};
use eyre::*;
use futures::SinkExt;
use futures::StreamExt;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::net::TcpStream;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use tracing::*;

pub struct WsClient {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    seq: u32,
}
impl WsClient {
    pub async fn new(connect_addr: &str) -> Result<Self> {
        let (ws_stream, _) = connect_async(connect_addr).await?;
        Ok(Self {
            stream: ws_stream,
            seq: 0,
        })
    }
    pub async fn send_req(&mut self, method: u32, params: impl Serialize) -> Result<()> {
        self.seq += 1;
        self.stream
            .send(tokio_tungstenite::tungstenite::Message::Text(
                serde_json::to_string(&WsRequestGeneric {
                    method,
                    seq: self.seq,
                    params,
                })?,
            ))
            .await?;
        Ok(())
    }
    pub async fn recv_raw(&mut self) -> Result<WsResponse> {
        let msg = self
            .stream
            .next()
            .await
            .ok_or(eyre!("Connection closed"))??;
        let resp: WsResponse = serde_json::from_str(&msg.to_string())?;
        Ok(resp)
    }
    pub async fn recv_resp<T: DeserializeOwned>(&mut self) -> Result<T> {
        loop {
            let msg = self
                .stream
                .next()
                .await
                .ok_or(eyre!("Connection closed"))??;
            match msg {
                Message::Text(text) => {
                    let resp: WsResponseGeneric<T> = serde_json::from_str(&text)?;
                    match resp {
                        WsResponseGeneric::Immediate(resp) if resp.seq == self.seq => {
                            return Ok(resp.params);
                        }
                        WsResponseGeneric::Immediate(resp) => {
                            bail!("Seq mismatch this: {} got: {}", self.seq, resp.seq)
                        }
                        WsResponseGeneric::Stream(_) => {
                            debug!("expect immediate response, got stream")
                        }
                        WsResponseGeneric::Forwarded(_) => {
                            debug!("expect immediate response, got forwarded")
                        }
                        WsResponseGeneric::Log(WsLogResponse {
                            log_id,
                            level,
                            message,
                            ..
                        }) => match level {
                            LogLevel::Error => error!(?log_id, "{}", message),
                            LogLevel::Warn => warn!(?log_id, "{}", message),
                            LogLevel::Info => info!(?log_id, "{}", message),
                            LogLevel::Debug => debug!(?log_id, "{}", message),
                            LogLevel::Trace => trace!(?log_id, "{}", message),
                            LogLevel::Off => {}
                        },
                        WsResponseGeneric::Error(err) => bail!(
                            "Error: {} {:?} {:?}",
                            err.code,
                            ErrorCode::new(err.code)
                                .canonical_reason()
                                .unwrap_or("UNKNOWN"),
                            err.reason
                        ),
                    }
                }
                Message::Close(_) => {
                    self.stream.close(None).await?;
                    bail!("Connection closed")
                }
                _ => {}
            }
        }
    }
    pub async fn request<T: DeserializeOwned>(
        &mut self,
        method: u32,
        params: impl Serialize,
    ) -> Result<T> {
        self.send_req(method, params).await?;
        self.recv_resp().await
    }
}
