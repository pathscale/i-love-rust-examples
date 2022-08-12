use async_compat::{Compat, CompatExt};
use async_tungstenite::tungstenite::error::ProtocolError;
use async_tungstenite::tungstenite::Error as WsError;
use async_tungstenite::tungstenite::Message;
use async_tungstenite::WebSocketStream;
use eyre::*;
use futures::{SinkExt, Stream};
use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Poll;
use tokio::sync::mpsc;
use tracing::*;

use crate::model::WsEndpointSchema;
use crate::ws::basics::{
    AsyncWsResponse, Connection, RequestHandlerRaw, WsRequest, WsResponseError,
};
use crate::ws::{request_error_to_resp, JsonVerifier, VerifyProtocol, WsEndpoint};
use tokio::net::TcpStream;

pub struct WsStream {
    stream: WebSocketStream<Compat<TcpStream>>,
}
#[derive(Default)]
pub struct WebsocketHandler {
    pub handlers: HashMap<u32, WsEndpoint>,
    pub connection: HashMap<u32, Arc<WsStream>>,
}

impl WebsocketHandler {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn add_handler_raw(
        &mut self,
        schema: WsEndpointSchema,
        handler: Arc<dyn RequestHandlerRaw>,
    ) {
        self.handlers
            .insert(schema.code, WsEndpoint { schema, handler });
    }
}
struct ReadWrite<'a> {
    send_rx: &'a mut mpsc::Receiver<Message>,
    stream: &'a mut WsStream,
}

enum WsEvent {
    ReceiveTerminated,
    TransmitTerminated,
    Request(Message),
    Response(Message),
    Error(WsError),
}

impl<'a> Future for ReadWrite<'a> {
    type Output = WsEvent;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut();
        match Pin::new(&mut this.stream.stream).poll_next(cx) {
            Poll::Ready(Some(x)) => {
                let x: Result<Message, WsError> = x;
                return match x {
                    Ok(msg) => Poll::Ready(WsEvent::Request(msg)),
                    Err(err) => Poll::Ready(WsEvent::Error(err)),
                };
            }
            Poll::Ready(None) => return Poll::Ready(WsEvent::ReceiveTerminated),
            Poll::Pending => {}
        }
        match this.send_rx.poll_recv(cx) {
            Poll::Ready(Some(msg)) => {
                return Poll::Ready(WsEvent::Response(msg));
            }
            Poll::Ready(None) => return Poll::Ready(WsEvent::TransmitTerminated),
            Poll::Pending => {}
        }

        Poll::Pending
    }
}

fn wrap_ws_error<T>(err: Result<T, WsError>) -> Result<T> {
    err.map_err(|x| eyre!(x))
}

impl WebsocketHandler {
    // 0login, 1pepe_pablo, 2AHJQ6X1H68SK8D9P6WW0, 31, 424787297130491616, 5android
    async fn handle_headers(header: &str, conn: &mut Connection) -> Result<()> {
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

            Ok(())
        } else {
            bail!("Could not process method {}", method)
        }
    }
    async fn handle_request(self: Arc<Self>, addr: SocketAddr, stream: TcpStream) {
        let result: Result<(), Error> = async {
            let (tx, mut rx) = mpsc::channel(1);
            let stream = wrap_ws_error(
                async_tungstenite::accept_hdr_async(stream.compat(), VerifyProtocol { tx }).await,
            )?;
            let mut stream = WsStream { stream };
            let (msg_tx, mut msg_rx) = mpsc::channel(100);
            let mut conn = Connection {
                connection_id: 0,
                user_id: 0,
                role: 0,
                send_tx: msg_tx.clone(),
            };
            // TODO: handle connections
            let headers = rx
                .recv()
                .await
                .ok_or_else(|| eyre!("Failed to receive ws headers"))?;
            Self::handle_headers(&headers, &mut conn).await?;
            let conn = Arc::new(conn);
            loop {
                let read_write = ReadWrite {
                    send_rx: &mut msg_rx,
                    stream: &mut stream,
                };
                match read_write.await {
                    WsEvent::ReceiveTerminated
                    | WsEvent::Error(WsError::Protocol(
                        ProtocolError::ResetWithoutClosingHandshake,
                    )) => {
                        info!(?addr, "Receive side terminated");
                        let _ = stream.stream.close(None).await;
                        break;
                    }
                    WsEvent::TransmitTerminated => {
                        info!(?addr, "Transmit side terminated");
                        let _ = stream.stream.close(None).await;
                        break;
                    }
                    WsEvent::Response(resp) => {
                        stream.stream.send(resp).await?;
                    }
                    WsEvent::Error(err) => {
                        stream.stream.close(None).await?;
                        return wrap_ws_error(Err(err));
                    }
                    WsEvent::Request(req) => {
                        let obj: Result<WsRequest> = match req {
                            Message::Text(t) => serde_json::from_str(&t)?,
                            Message::Binary(b) => serde_json::from_slice(&b)?,
                            Message::Ping(_) => {
                                continue;
                            }
                            Message::Pong(_) => {
                                continue;
                            }
                            Message::Close(_) => {
                                info!(?addr, "Receive side terminated");
                                stream.stream.close(None).await?;
                                break;
                            }
                        };
                        let req = match obj {
                            Ok(req) => req,
                            Err(err) => {
                                stream
                                    .stream
                                    .send(Message::Text(serde_json::to_string(
                                        &request_error_to_resp(0, 0, 0, err),
                                    )?))
                                    .await?;

                                continue;
                            }
                        };
                        let handler = self.handlers.get(&req.method);
                        let handler = match handler {
                            Some(handler) => handler,
                            None => {
                                stream
                                    .stream
                                    .send(Message::Text(serde_json::to_string(
                                        &request_error_to_resp(
                                            req.method,
                                            400,
                                            req.seq,
                                            eyre!(
                                                "Could not find handler for method code {}",
                                                req.method
                                            ),
                                        ),
                                    )?))
                                    .await?;
                                continue;
                            }
                        };
                        let result = handler.handler.handle(Arc::clone(&conn), req);
                        match result {
                            AsyncWsResponse::Sync(resp) => {
                                let j = resp.dump_json();
                                stream.stream.send(Message::Text(j)).await?;
                            }
                            AsyncWsResponse::Async(fut) => {
                                let tx = msg_tx.clone();
                                tokio::spawn(async move {
                                    let resp = fut.await;
                                    let j = resp.dump_json();
                                    let _ = tx.send(Message::Text(j)).await;
                                });
                            }
                        }
                    }
                }
            }
            Ok(())
        }
        .await;
        if let Err(err) = result {
            error!(?addr, "Error while processing {:?}", err)
        }
    }
    pub async fn listen(self: Arc<Self>, addr: &str) -> Result<()> {
        info!("Listening on {}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        loop {
            let (stream, addr) = listener.accept().await?;
            info!("Accepted stream from {}", addr);
            tokio::spawn(Arc::clone(&self).handle_request(addr, stream));
        }
    }
}
