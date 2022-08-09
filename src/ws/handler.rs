use std::collections::HashMap;
use std::future::Future;
use std::net::{SocketAddr};
use std::pin::Pin;
use std::sync::{Arc};
use std::task::Poll;
use tokio::sync::mpsc;
use eyre::*;
use async_compat::{Compat, CompatExt};
use async_tungstenite::tungstenite::Error as WsError;
use futures::{SinkExt, Stream};
use tracing::*;
use async_tungstenite::tungstenite::Message;
use async_tungstenite::WebSocketStream;

use tokio::net::TcpStream;
use crate::ws::basics::{AsyncWsResponse, Connection, WsRequest, WsResponseError};
use crate::ws::{JsonVerifier, VerifyProtocol, WsEndpoint};

pub struct WsStream {
    stream: WebSocketStream<Compat<TcpStream>>,
}

pub struct WebsocketHandler {
    pub handlers: HashMap<u32, WsEndpoint>,
    pub connection: HashMap<u32, Arc<WsStream>>,
    pub verifier: JsonVerifier,
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
                    Ok(msg) => { Poll::Ready(WsEvent::Request(msg)) }
                    Err(err) => { Poll::Ready(WsEvent::Error(err)) }
                };
            }
            Poll::Ready(None) => return Poll::Ready(WsEvent::ReceiveTerminated),
            Poll::Pending => {}
        }
        match this.send_rx.poll_recv(cx) {
            Poll::Ready(Some(msg)) => { return Poll::Ready(WsEvent::Response(msg)); }
            Poll::Ready(None) => return Poll::Ready(WsEvent::TransmitTerminated),
            Poll::Pending => {}
        }

        Poll::Pending
    }
}

fn wrap_error<T>(err: Result<T, WsError>) -> Result<T> {
    err.map_err(|x| eyre!(x))
}

impl WebsocketHandler {
    async fn handle_headers(_header: &str, conn: &mut Connection) -> Result<()> {
        conn.user_id = 1;
        conn.role = 1;
        Ok(())
    }
    async fn handle_request(self: Arc<Self>, addr: SocketAddr, stream: TcpStream) {
        let result: Result<(), Error> = async {
            let (tx, mut rx) = mpsc::channel(1);
            let stream = wrap_error(async_tungstenite::accept_hdr_async(stream.compat(), VerifyProtocol {
                tx
            }).await)?;
            let mut stream = WsStream {
                stream
            };
            let (msg_tx, mut msg_rx) = mpsc::channel(100);
            let mut conn = Connection {
                connection_id: 0,
                user_id: 0,
                role: 0,
                send_tx: msg_tx.clone(),
            };
            // TODO: handle connections
            let headers = rx.recv().await.ok_or_else(|| eyre!("Failed to receive ws headers"))?;
            Self::handle_headers(&headers, &mut conn).await?;
            let conn = Arc::new(conn);
            loop {
                let read_write = ReadWrite {
                    send_rx: &mut msg_rx,
                    stream: &mut stream,
                };
                match read_write.await {
                    WsEvent::ReceiveTerminated => {
                        info!(?addr, "Receive side terminated");
                        break;
                    }
                    WsEvent::TransmitTerminated => {
                        info!(?addr, "Transmit side terminated");
                        break;
                    }
                    WsEvent::Response(resp) => {
                        stream.stream.send(resp).await?;
                    }
                    WsEvent::Error(err) => {
                        wrap_error(Err(err))?;
                    }
                    WsEvent::Request(req) => {
                        let obj: Result<WsRequest> = match req {
                            Message::Text(t) => { self.verifier.try_parse(t.as_bytes()) }
                            Message::Binary(b) => { self.verifier.try_parse(b.as_ref()) }
                            Message::Ping(_) => {
                                continue;
                            }
                            Message::Pong(_) => {
                                continue;
                            }
                            Message::Close(_) => {
                                info!(?addr, "Receive side terminated");
                                break;
                            }
                        };
                        let req = match obj {
                            Ok(req) => { req }
                            Err(x) => {
                                // request is not valid json
                                stream.stream.send(Message::Text(serde_json::to_string(&WsResponseError {
                                    method: 0,
                                    code: 0,
                                    seq: 0,
                                    reason: x.to_string(),
                                })?)).await?;
                                continue;
                            }
                        };
                        let handler = self.handlers.get(&req.method);
                        let handler = match handler {
                            Some(handler) => {handler}
                            None => {
                                stream.stream.send(Message::Text(serde_json::to_string(&WsResponseError {
                                    method: 0,
                                    code: 0,
                                    seq: req.seq,
                                    reason: format!("Could not find handler for method code {}", req.method),
                                })?)).await?;
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
        }.await;
        if let Err(err) = result {
            error!(?addr, "Error while processing {:?}", err)
        }
    }
    pub async fn listen(self: Arc<Self>) -> Result<()> {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:4444").await?;
        loop {
            let (stream, addr) = listener.accept().await?;
            info!("Accepted stream from {}", addr);
            tokio::spawn(Arc::clone(&self).handle_request(addr, stream));
        }
    }
}
