use async_tungstenite::tungstenite::Message;
use eyre::*;
use futures::future::BoxFuture;
use futures::FutureExt;
use serde::de::DeserializeOwned;
use serde::*;
use std::fmt::{Debug, Display};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WsRequest {
    pub method: u32,
    pub seq: u32,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WsRequestGeneric<Req> {
    pub method: u32,
    pub seq: u32,
    pub data: Req,
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
pub struct WsSuccessResponseGeneric<Params> {
    pub method: u32,
    pub seq: u32,
    pub params: Params,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WsStreamResponseGeneric<Params> {
    pub method: u32,
    pub stream_seq: u32,
    pub resource: String,
    pub data: Params,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WsResponse {
    Immediate(WsSuccessResponse),
    Stream(WsStreamResponse),
    Forwarded(WsForwardedResponse),
    Error(WsResponseError),
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WsResponseGeneric<Resp> {
    Immediate(WsSuccessResponseGeneric<Resp>),
    Stream(WsStreamResponseGeneric<Resp>),
    Forwarded(WsForwardedResponse),
    Error(WsResponseError),
}

impl WsResponse {
    pub fn dump_json(&self) -> String {
        match self {
            WsResponse::Immediate(x) => serde_json::to_string(x),
            WsResponse::Stream(x) => serde_json::to_string(x),
            WsResponse::Forwarded(x) => serde_json::to_string(x),
            WsResponse::Error(x) => serde_json::to_string(x),
        }
        .expect("Failed to dump json(impossible)")
    }
}
impl<Resp: Serialize> WsResponseGeneric<Resp> {
    pub fn generalize(self) -> Result<WsResponse> {
        match self {
            WsResponseGeneric::Immediate(x) => Ok(WsResponse::Immediate(WsSuccessResponse {
                method: x.method,
                seq: x.seq,
                params: serde_json::to_value(&x.params)?,
            })),
            WsResponseGeneric::Stream(x) => Ok(WsResponse::Stream(WsStreamResponse {
                method: x.method,
                stream_seq: x.stream_seq,
                resource: x.resource,
                data: serde_json::to_value(&x.data)?,
            })),
            WsResponseGeneric::Forwarded(x) => Ok(WsResponse::Forwarded(x)),
            WsResponseGeneric::Error(x) => Ok(WsResponse::Error(x)),
        }
    }
}
pub enum AsyncWsResponse {
    Sync(WsResponse),
    Async(BoxFuture<'static, WsResponse>),
}
pub enum AsyncWsResponseGeneric<Resp> {
    Sync(WsResponseGeneric<Resp>),
    Async(BoxFuture<'static, WsResponseGeneric<Resp>>),
}
pub fn get_log_id() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as _
}
pub fn internal_error_to_resp(method: u32, code: u32, seq: u32, err: Error) -> WsResponse {
    let log_id = get_log_id();
    error!(?log_id, "Internal error: {:?}", err);
    WsResponse::Error(WsResponseError {
        method,
        code,
        seq,
        reason: format!("Internal error: log_id={}", log_id),
    })
}
pub fn request_error_to_resp<E: Display + Debug>(
    method: u32,
    code: u32,
    seq: u32,
    err: E,
) -> WsResponse {
    let log_id = get_log_id();
    debug!(?log_id, "Request error: {:?}", err);
    WsResponse::Error(WsResponseError {
        method,
        code,
        seq,
        reason: format!("Request error log_id={}: {}", log_id, err),
    })
}
impl<Resp: Serialize + 'static> AsyncWsResponseGeneric<Resp> {
    pub fn generalize(self, method: u32, seq: u32) -> AsyncWsResponse {
        match self {
            AsyncWsResponseGeneric::Sync(resp) => match resp.generalize() {
                Ok(ok) => AsyncWsResponse::Sync(ok),
                Err(err) => AsyncWsResponse::Sync(internal_error_to_resp(method, 500, seq, err)),
            },

            AsyncWsResponseGeneric::Async(fut) => AsyncWsResponse::Async(
                async move {
                    match fut.await.generalize() {
                        Ok(ok) => ok,
                        Err(err) => internal_error_to_resp(method, 500, seq, err),
                    }
                }
                .boxed(),
            ),
        }
    }
}
pub trait RequestHandlerRaw: Send + Sync {
    fn handle(&self, conn: Arc<Connection>, req: WsRequest) -> AsyncWsResponse;
}

pub trait RequestHandler: Send + Sync {
    type Request: DeserializeOwned;
    type Response: Serialize + 'static;
    fn handle(
        &self,
        conn: Arc<Connection>,
        req: WsRequestGeneric<Self::Request>,
    ) -> Result<AsyncWsResponseGeneric<Self::Response>>;
}

impl<T: RequestHandler> RequestHandlerRaw for T {
    fn handle(&self, conn: Arc<Connection>, req: WsRequest) -> AsyncWsResponse {
        let data: T::Request = match serde_json::from_value(req.data) {
            Ok(data) => data,
            Err(err) => {
                return AsyncWsResponse::Sync(request_error_to_resp(req.method, 400, req.seq, err))
            }
        };
        let req1 = WsRequestGeneric {
            method: req.method,
            seq: req.seq,
            data,
        };
        let resp = RequestHandler::handle(self, conn, req1);
        match resp {
            Ok(ok) => ok.generalize(req.method, req.seq),
            Err(err) => {
                AsyncWsResponse::Sync(internal_error_to_resp(req.method, 500, req.seq, err))
            }
        }
    }
}

pub trait AsyncRequestHandler: Send + Sync {
    type Request: DeserializeOwned;
    type Response: Serialize + 'static;
    fn handle(
        &self,
        conn: Arc<Connection>,
        req: WsRequestGeneric<Self::Request>,
    ) -> BoxFuture<'static, Result<Self::Response>>;
}
impl<T: AsyncRequestHandler> RequestHandler for T {
    type Request = T::Request;
    type Response = T::Response;

    fn handle(
        &self,
        conn: Arc<Connection>,
        req: WsRequestGeneric<Self::Request>,
    ) -> Result<AsyncWsResponseGeneric<Self::Response>> {
        let method = req.method;
        let seq = req.seq;
        let resp = AsyncRequestHandler::handle(self, conn, req);
        Ok(AsyncWsResponseGeneric::Async(
            async move {
                match resp.await {
                    Ok(ok) => WsResponseGeneric::Immediate(WsSuccessResponseGeneric {
                        method,
                        seq,
                        params: ok,
                    }),
                    Err(err) => {
                        let log_id = get_log_id();
                        debug!(?log_id, "Request error: {:?}", err);
                        WsResponseGeneric::Error(WsResponseError {
                            method,
                            code: 400,
                            seq,
                            reason: format!("Request error log_id={}: {}", log_id, err),
                        })
                    }
                }
            }
            .boxed(),
        ))
    }
}
