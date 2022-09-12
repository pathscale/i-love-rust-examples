use crate::database::SimpleDbClient;
use crate::error_code::ErrorCode;
use crate::log::LogLevel;
use crate::ws::*;
use dashmap::DashMap;
use eyre::*;
use reqwest::StatusCode;
use serde::*;
use std::any::Any;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tracing::*;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoResp;
impl Display for NoResp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("NoResp")
    }
}
impl std::error::Error for NoResp {}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomError {
    pub code: ErrorCode,
    pub reason: String,
}
impl CustomError {
    pub fn new(code: impl Into<ErrorCode>, reason: String) -> Self {
        Self {
            code: code.into(),
            reason,
        }
    }
    pub fn from_sql_error(err: &str, msg: impl Display) -> Result<Self> {
        let code = u32::from_str_radix(err, 36)?;
        let error_code = ErrorCode::new(code);
        let this = Self {
            code: error_code,
            reason: format!(
                "{} {} {}",
                err,
                error_code.canonical_reason().unwrap_or(""),
                msg
            ),
        };

        Ok(this)
    }
}
impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.reason)
    }
}
impl std::error::Error for CustomError {}
#[derive(Copy, Clone)]
pub struct RequestContext {
    pub connection_id: u32,
    pub user_id: i64,
    pub seq: u32,
    pub method: u32,
    pub log_id: u64,
}
#[derive(Clone)]
pub struct Toolbox {
    db: Option<SimpleDbClient>,
    values: Arc<DashMap<String, Arc<dyn Any + Send + Sync>>>,
    sender: mpsc::Sender<WsMessage>,
    tasks: Option<Arc<Mutex<Vec<tokio::task::JoinHandle<()>>>>>,
}

impl Toolbox {
    pub fn new(sender: mpsc::Sender<WsMessage>) -> Self {
        Self {
            db: None,
            values: Arc::new(Default::default()),
            sender,
            tasks: None,
        }
    }
    pub fn set_db(&mut self, db: SimpleDbClient) {
        self.db = Some(db);
    }
    pub fn get_db<T: From<SimpleDbClient>>(&self) -> T {
        T::from(self.db.as_ref().expect("Db not Initialized").clone())
    }
    pub fn set_value(&mut self, key: &str, value: Arc<dyn Any + Send + Sync>) {
        self.values.insert(key.to_string(), value);
    }
    pub fn get_value<T: 'static>(&self, key: &str) -> Option<Arc<T>> {
        self.values.get(key).map(|x| {
            x.downcast_ref::<Arc<T>>()
                .cloned()
                .expect("Cannot convert type")
        })
    }
    pub fn send(&self, ctx: &RequestContext, resp: WsResponse) {
        self.sender
            .try_send(WsMessage {
                connection_id: ctx.connection_id,
                message: resp,
            })
            .map_err(|e| {
                error!("Cannot send message, queue full: {}", e);
            })
            .ok();
    }
    pub fn send_error(&self, ctx: &RequestContext, code: ErrorCode, err: Error) {
        self.send(ctx, internal_error_to_resp(ctx, code, err));
    }
    pub fn send_log(&self, ctx: &RequestContext, level: LogLevel, msg: impl Into<String>) {
        self.send(
            ctx,
            WsResponse::Log(WsLogResponse {
                seq: ctx.seq,
                log_id: ctx.log_id,
                level,
                message: msg.into(),
            }),
        );
    }
    pub fn collect_tasks(&mut self, f: impl FnOnce(&Self)) -> Vec<tokio::task::JoinHandle<()>> {
        self.tasks.replace(Arc::new(Mutex::new(vec![])));
        f(self);
        let tasks = self.tasks.take().unwrap();
        let mut tasks = tasks.lock().unwrap();
        let mut result = vec![];
        std::mem::swap(&mut result, &mut *tasks);
        self.tasks = None;
        result
    }
    pub fn spawn_response<Resp: Send + Serialize>(
        &self,
        ctx: RequestContext,
        f: impl Future<Output = Result<Resp>> + Send + 'static,
    ) {
        let sender = self.sender.clone();
        #[allow(unused_variables)]
        let RequestContext {
            connection_id,
            user_id,
            seq,
            method,
            log_id,
        } = ctx;
        let t = tokio::spawn(async move {
            let resp = f.await;
            let resp = match resp {
                Ok(ok) => WsResponse::Immediate(WsSuccessResponse {
                    method,
                    seq,
                    params: serde_json::to_value(ok).expect("Failedto serialize response"),
                }),
                Err(err) if err.downcast_ref::<NoResp>().is_some() => {
                    return;
                }

                Err(err0) if err0.downcast_ref::<tokio_postgres::Error>().is_some() => {
                    let err = err0.downcast_ref::<tokio_postgres::Error>().unwrap();
                    if let Some(code) = err.code() {
                        if let Ok(err) = CustomError::from_sql_error(code.code(), &err0) {
                            request_error_to_resp(&ctx, err.code, err)
                        } else {
                            internal_error_to_resp(
                                &ctx,
                                StatusCode::INTERNAL_SERVER_ERROR.into(),
                                err0,
                            )
                        }
                    } else {
                        internal_error_to_resp(&ctx, StatusCode::INTERNAL_SERVER_ERROR.into(), err0)
                    }
                }
                Err(err) if err.downcast_ref::<CustomError>().is_some() => {
                    let err = err.downcast::<CustomError>().unwrap();
                    request_error_to_resp(&ctx, err.code, err)
                }
                Err(err) => {
                    internal_error_to_resp(&ctx, StatusCode::INTERNAL_SERVER_ERROR.into(), err)
                }
            };
            let _ = sender
                .send(WsMessage {
                    connection_id,
                    message: resp,
                })
                .await;
        });
        if let Some(tasks) = &self.tasks {
            tasks.lock().unwrap().push(t);
        }
    }
}
