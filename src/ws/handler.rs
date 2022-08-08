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
use crate::ws::basics::{Connection};
use crate::ws::{VerifyProtocol, WsEndpoint};

pub struct WsStream {
    stream: WebSocketStream<Compat<TcpStream>>,
}

pub struct WebsocketHandler {
    pub handlers: HashMap<u32, WsEndpoint>,
    pub connection: HashMap<u32, Arc<WsStream>>,
}

struct ReadWrite<'a> {
    send_rx: &'a mut mpsc::Receiver<Message>,
    stream: &'a mut WebSocketStream<Compat<TcpStream>>,
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
        match Pin::new(&mut this.stream).poll_next(cx) {
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
            let mut stream = wrap_error(async_tungstenite::accept_hdr_async(stream.compat(), VerifyProtocol {
                tx
            }).await)?;
            let (msg_tx, mut msg_rx) = mpsc::channel(100);
            let mut conn = Connection {
                connection_id: 0,
                user_id: 0,
                role: 0,
                send_tx: msg_tx,
            };
            // TODO: handle connections
            let headers = rx.recv().await.ok_or_else(|| eyre!("Failed to receive ws headers"))?;
            Self::handle_headers(&headers, &mut conn).await?;
            // TODO: receive message from stream
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
                    WsEvent::Request(_req) => {
                        // TODO: verify and parse
                        // TODO: execute
                    }
                    WsEvent::Response(resp) => {
                        stream.send(resp).await?;
                    }
                    WsEvent::Error(err) => {
                        wrap_error(Err(err))?;
                    }
                }
                // match msg_rx.recv().await {
                //     Some(WsResponse::Immediate {
                //              method,
                //              seq,
                //              params
                //          }) => {
                //         stream.as_mut().start_send(Message::Text(serde_json::to_string(&json! {{
                //             "seq": seq,
                //             "method": method,
                //             "params": params,
                //         }})?))?;
                //     }
                //     Some(WsResponse::Error {
                //              code, seq, reason
                //          }) => {
                //         stream.as_mut().start_send(Message::Text(serde_json::to_string(&json! {{
                //             "seq": seq,
                //             "code": code,
                //             "reason": reason,
                //         }})?))?;
                //     }
                //     None => {
                //         break;
                //     }
                //     _ => todo!(),
                // }
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
