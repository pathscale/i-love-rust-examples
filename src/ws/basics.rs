use std::sync::Arc;
use async_tungstenite::tungstenite::Message;
use futures::future::BoxFuture;
use serde::*;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WsRequest {
    pub method: u32,
    pub seq: u32,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WsResponseError {
    pub method: u32,
    pub code: u32,
    pub seq: u32,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub connection_id: u32,
    pub user_id: u32,
    pub role: u32,
    pub send_tx: tokio::sync::mpsc::Sender<Message>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
    pub log_id: u32,
    pub connection_id: u32,
    pub user_id: u32,
    pub role: u32,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WsSuccessResponse {
    pub method: u32,
    pub seq: u32,
    pub params: serde_json::Value,

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WsStreamResponse {
    pub method: u32,
    pub stream_seq: u32,
    pub resource: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WsForwardedResponse {
    pub method: u32,
    pub seq: u32,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WsResponse {
    Immediate(WsSuccessResponse),
    Stream(WsStreamResponse),
    Forwarded(WsForwardedResponse),
    Error(WsResponseError),
}

impl WsResponse {
    pub fn dump_json(&self) -> String {
        match self {
            WsResponse::Immediate(x) => { serde_json::to_string(x) }
            WsResponse::Stream(x) => { serde_json::to_string(x) }
            WsResponse::Forwarded(x) => { serde_json::to_string(x) }
            WsResponse::Error(x) => { serde_json::to_string(x) }
        }.expect("Failed to dump json(impossible)")
    }
}

pub enum AsyncWsResponse {
    Sync(WsResponse),
    Async(BoxFuture<'static, WsResponse>),
}

pub trait RequestHandler: Send + Sync {
    fn handle(&self, conn: Arc<Connection>, req: WsRequest) -> AsyncWsResponse;
}