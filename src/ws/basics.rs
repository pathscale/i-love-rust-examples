use std::borrow::Cow;
use async_tungstenite::stream::Stream;
use eyre::*;
use futures::future::BoxFuture;
use serde::*;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WsRequest {
    pub method: u32,
    pub seq: u32,
    pub data: serde_json::Value,
}

pub struct Connection {
    pub connection_id: u32,
    pub user_id: u32,
    pub role: u32,
    pub send_tx: tokio::sync::mpsc::Sender<WsResponse>
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Context {
    pub log_id: u32,
    pub connection_id: u32,
    pub user_id: u32,
    pub role: u32,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WsResponse {
    Immediate{
        method: u32,
        seq: u32,
        params: serde_json::Value,
    },
    Stream(serde_json::Value),
    Forwarded(serde_json::Value),
    Error {
        code: u32,
        seq: u32,
        reason: String,
    },
}

pub enum AsyncWsResponse {
    Sync(WsResponse),
    Async(BoxFuture<'static, WsResponse>),
}

pub trait RequestHandler {
    fn handle(&self, conn: Connection, req: WsRequest) -> AsyncWsResponse;
}