use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use eyre::*;
use async_compat::{Compat, CompatExt};
use futures::Sink;
use tracing::level_filters::LevelFilter;
use tracing::*;
use pin_utils::pin_mut;
use async_tungstenite::tungstenite::handshake::server::{Callback, Request, Response, ErrorResponse};
use async_tungstenite::tungstenite::Message;
use async_tungstenite::WebSocketStream;
use serde_json::json;
use tokio::net::TcpStream;
use tracing_subscriber::fmt::format::Compact;
use crate::analyze::{thread2, thread3};
use crate::logger::setup_logs;
use crate::poll_data::fetch_from_polygon;
use crate::utils::{error_handled, error_handled_sync};
use crate::ws::basics::{Connection, WsRequest, WsResponse};
use crate::YourDataStruct;
use crate::ws::{VerifyProtocol, WsEndpoint};

pub struct WsStream {
    stream: WebSocketStream<Compat<TcpStream>>,
}

pub struct WebsocketHandler {
    handlers: HashMap<u32, WsEndpoint>,
    connection: HashMap<u32, Arc<WsStream>>,
}

impl WebsocketHandler {
    async fn handle_headers(header: &str, conn: &mut Connection) -> Result<()> {
        conn.user_id = 1;
        conn.role = 1;
        Ok(())
    }
    async fn handle_request(addr: SocketAddr, stream: TcpStream) {
        let result = async {
            let (tx, mut rx) = tokio::sync::mpsc::channel(1);
            let stream = async_tungstenite::accept_hdr_async(stream.compat(), VerifyProtocol {
                tx
            }).await?;
            pin_mut!(stream);
            let (msg_tx, mut msg_rx) = tokio::sync::mpsc::channel(100);
            let mut conn = Connection {
                connection_id: 0,
                user_id: 0,
                role: 0,
                send_tx: msg_tx
            };
            let headers = rx.recv().await.ok_or_else(|| eyre!("Failed to receive ws headers"))?;
            Self::handle_headers(&headers, &mut conn).await?;
            // TODO: receive message from stream
            loop {
                match msg_rx.recv().await {
                    Some(WsResponse::Immediate {
                             method,
                             seq,
                             params
                         }) => {
                        stream.as_mut().start_send(Message::Text(serde_json::to_string(&json! {{
                            "seq": seq,
                            "method": method,
                            "params": params,
                        }})?))?;
                    }
                    Some(WsResponse::Error {
                             code, seq, reason
                         }) => {
                        stream.as_mut().start_send(Message::Text(serde_json::to_string(&json! {{
                            "seq": seq,
                            "code": code,
                            "reason": reason,
                        }})?))?;
                    }
                    None => {
                        break;
                    }
                    _ => todo!(),
                }
            }
            Ok::<(), Error>(())
        }.await;
        if let Err(err) = result {
            error!(?addr, "Error while processing {}", err)
        }
    }
    async fn listen(&self) -> Result<()> {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:4444").await?;
        loop {
            let (stream, addr) = listener.accept().await?;
            info!("Accepted stream from {}", addr);
            tokio::spawn(Self::handle_request(addr, stream));
        }
    }
}
