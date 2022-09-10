use crate::error_code::ErrorCode;
use crate::log::LogLevel;
use crate::toolbox::{RequestContext, Toolbox};
use eyre::*;
use model::endpoint::EndpointSchema;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::*;
use std::fmt::{Debug, Display};
use std::net::IpAddr;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use tracing::{debug, error};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WsRequestGeneric<Req> {
    pub method: u32,
    pub seq: u32,
    pub params: Req,
}
pub type WsRequest = WsRequestGeneric<serde_json::Value>;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WsResponseError {
    pub method: u32,
    pub code: u32,
    pub seq: u32,
    pub reason: String,
}

#[derive(Debug)]
pub struct Connection {
    pub connection_id: u32,
    pub user_id: AtomicU32,
    pub role: AtomicU32,
    pub address: IpAddr,
    pub log_id: u64,
}

pub type WsSuccessResponse = WsSuccessResponseGeneric<serde_json::Value>;
pub type WsStreamResponse = WsStreamResponseGeneric<serde_json::Value>;

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
pub struct WsLogResponse {
    pub seq: u32,
    pub log_id: u64,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)] // order matters
pub enum WsResponseGeneric<Resp> {
    Immediate(WsSuccessResponseGeneric<Resp>),
    Stream(WsStreamResponseGeneric<Resp>),
    Error(WsResponseError),
    Log(WsLogResponse),
    Forwarded(WsForwardedResponse),
}

pub type WsResponse = WsResponseGeneric<serde_json::Value>;

pub fn internal_error_to_resp(ctx: &RequestContext, code: ErrorCode, err: Error) -> WsResponse {
    let log_id = ctx.log_id;
    error!(?log_id, "Internal error: {:?}", err);
    WsResponse::Error(WsResponseError {
        method: ctx.method,
        code: code.to_u32(),
        seq: ctx.seq,
        reason: format!("Internal error: log_id={}", log_id),
    })
}
pub fn request_error_to_resp<E: Display + Debug>(
    ctx: &RequestContext,
    code: ErrorCode,
    err: E,
) -> WsResponse {
    let log_id = ctx.log_id;

    debug!(?log_id, "Request error: {:?}", err);
    WsResponse::Error(WsResponseError {
        method: ctx.method,
        code: code.to_u32(),
        seq: ctx.seq,
        reason: format!("Request error log_id={}: {}", log_id, err),
    })
}

pub struct WsEndpoint {
    pub schema: EndpointSchema,
    pub handler: Arc<dyn RequestHandlerErased>,
}
pub trait RequestHandlerErased: Send + Sync {
    fn handle(&self, toolbox: &Toolbox, ctx: RequestContext, conn: Arc<Connection>, req: WsRequest);
}

pub trait RequestHandler: Send + Sync {
    type Request: DeserializeOwned;
    type Response: Serialize + 'static;
    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: WsRequestGeneric<Self::Request>,
    );
}

impl<T: RequestHandler> RequestHandlerErased for T {
    fn handle(
        &self,
        toolbox: &Toolbox,
        ctx: RequestContext,
        conn: Arc<Connection>,
        req: WsRequest,
    ) {
        let data: T::Request = match serde_json::from_value(req.params) {
            Ok(data) => data,
            Err(err) => {
                toolbox.send(
                    &ctx,
                    request_error_to_resp(&ctx, StatusCode::BAD_REQUEST.into(), err),
                );
                return;
            }
        };
        let req1 = WsRequestGeneric {
            method: req.method,
            seq: req.seq,
            params: data,
        };
        RequestHandler::handle(self, toolbox, ctx, conn, req1)
    }
}
