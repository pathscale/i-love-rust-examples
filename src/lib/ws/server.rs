use crate::database::SimpleDbClient;
use dashmap::DashMap;
use eyre::*;
use futures::stream::{SplitSink, SplitStream};
use futures::SinkExt;
use futures::StreamExt;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::net::SocketAddr;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::sync::mpsc;
use tokio_rustls::TlsAcceptor;
use tokio_tungstenite::tungstenite::error::ProtocolError;
use tokio_tungstenite::tungstenite::http::StatusCode;
use tokio_tungstenite::tungstenite::Error as WsError;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tracing::*;

use crate::config::AppConfig;
use crate::toolbox::{RequestContext, Toolbox};
use crate::utils::{get_conn_id, get_log_id};
use crate::ws::basics::{Connection, RequestHandlerErased, WsRequest};
use crate::ws::{
    request_error_to_resp, AuthController, SimpleAuthContoller, VerifyProtocol, WsEndpoint,
    WsResponse,
};
use model::endpoint::EndpointSchema;
use pem::parse;
use std::fs;
pub struct WsStream<S> {
    ws_sink: SplitSink<WebSocketStream<S>, Message>,
    conn: Arc<Connection>,
}

pub struct WsMessage {
    pub connection_id: u32,
    pub message: WsResponse,
}
pub struct WebsocketServer {
    pub auth_controller: Arc<dyn AuthController>,
    pub handlers: HashMap<u32, WsEndpoint>,
    pub message_receiver: Option<mpsc::Receiver<WsMessage>>,
    pub toolbox: Toolbox,
    pub config: AppConfig,
}
#[derive(Default)]
pub struct WebsocketStates<S> {
    pub connection: DashMap<u32, WsStream<S>>,
}
impl<S> WebsocketStates<S> {
    pub fn new() -> Self {
        Self {
            connection: Default::default(),
        }
    }
}
impl Default for WebsocketServer {
    fn default() -> Self {
        let (msg_tx, msg_rx) = mpsc::channel(100);

        Self {
            auth_controller: Arc::new(SimpleAuthContoller),
            handlers: Default::default(),
            message_receiver: Some(msg_rx),
            toolbox: Toolbox::new(msg_tx),
            config: Default::default(),
        }
    }
}
fn wrap_ws_error<T>(err: Result<T, WsError>) -> Result<T> {
    err.map_err(|x| eyre!(x))
}

impl WebsocketServer {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }
    pub fn add_auth_controller(&mut self, controller: Arc<dyn AuthController>) {
        self.auth_controller = controller;
    }
    pub fn add_database(&mut self, db: SimpleDbClient) {
        self.toolbox.set_db(db);
    }
    pub fn get_toolbox(&self) -> Toolbox {
        self.toolbox.clone()
    }
    pub fn add_handler(
        &mut self,
        schema: EndpointSchema,
        handler: impl RequestHandlerErased + 'static,
    ) {
        self.add_handler_erased(schema, Arc::new(handler))
    }
    pub fn add_handler_erased(
        &mut self,
        schema: EndpointSchema,
        handler: Arc<dyn RequestHandlerErased>,
    ) {
        let old = self
            .handlers
            .insert(schema.code, WsEndpoint { schema, handler });
        if let Some(old) = old {
            panic!(
                "Overwriting handler for endpoint {} {}",
                old.schema.code, old.schema.name
            );
        }
    }
    async fn handle_request<S: AsyncRead + AsyncWrite + Unpin + Send + 'static>(
        self: Arc<Self>,
        addr: SocketAddr,
        states: Arc<WebsocketStates<S>>,
        stream: S,
    ) {
        let result: Result<()> = async move {
            let (tx, mut rx) = mpsc::channel(1);
            let hs = tokio_tungstenite::accept_hdr_async(stream, VerifyProtocol { tx }).await;
            let mut stream = wrap_ws_error(hs)?;
            let conn = Arc::new(Connection {
                connection_id: get_conn_id(),
                user_id: Default::default(),
                role: AtomicU32::new(0),
                address: addr.ip(),
                log_id: get_log_id(),
            });
            let headers = rx
                .recv()
                .await
                .ok_or_else(|| eyre!("Failed to receive ws headers"))?;
            let auth_result = self.auth_controller.auth(headers, Arc::clone(&conn)).await;
            if let Err(err) = auth_result {
                let resp = request_error_to_resp(
                    &RequestContext {
                        connection_id: conn.connection_id,
                        user_id: 0,
                        seq: 0,
                        method: 0,
                        log_id: conn.log_id,
                    },
                    StatusCode::BAD_REQUEST.into(),
                    err,
                );
                let _ = stream
                    .send(Message::Text(serde_json::to_string(&resp)?))
                    .await;
                return Ok(());
            }
            let (ws_sink, ws_stream) = stream.split();

            let stream = WsStream {
                ws_sink: ws_sink,
                conn,
            };
            let conn = Arc::clone(&stream.conn);
            states.connection.insert(conn.connection_id, stream);
            tokio::spawn(Arc::clone(&self).recv_msg(conn, states, ws_stream));
            Ok(())
        }
        .await;
        if let Err(err) = result {
            error!(?addr, "Error while processing {:?}", err)
        }
    }

    pub async fn recv_msg<S: AsyncRead + AsyncWrite + Unpin + Send + 'static>(
        self: Arc<Self>,
        conn: Arc<Connection>,
        states: Arc<WebsocketStates<S>>,
        mut reader: SplitStream<WebSocketStream<S>>,
    ) {
        let addr = conn.address;
        let context = RequestContext {
            connection_id: conn.connection_id,
            user_id: conn.get_user_id(),
            seq: 0,
            method: 0,
            log_id: conn.log_id,
        };
        while let Some(msg) = reader.next().await {
            match msg {
                Ok(req) => {
                    let obj: Result<WsRequest, _> = match req {
                        Message::Text(t) => {
                            debug!(?addr, "Handling request {}", t);

                            serde_json::from_str(&t)
                        }
                        Message::Binary(b) => {
                            debug!(?addr, "Handling request <BIN>");
                            serde_json::from_slice(&b)
                        }
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
                        _ => {
                            warn!(?addr, "Strange pattern {:?}", req);
                            continue;
                        }
                    };
                    let req = match obj {
                        Ok(req) => req,
                        Err(err) => {
                            self.toolbox.send(
                                &context,
                                request_error_to_resp(
                                    &context,
                                    StatusCode::BAD_REQUEST.into(),
                                    err,
                                ),
                            );
                            continue;
                        }
                    };
                    let context = RequestContext {
                        seq: req.seq,
                        method: req.method,
                        ..context
                    };
                    let handler = self.handlers.get(&req.method);
                    let handler = match handler {
                        Some(handler) => handler,
                        None => {
                            self.toolbox.send(
                                &context,
                                request_error_to_resp(
                                    &context,
                                    StatusCode::NOT_FOUND.into(),
                                    eyre!("Could not find handler for method code {}", req.method),
                                ),
                            );
                            continue;
                        }
                    };
                    handler
                        .handler
                        .handle(&self.toolbox, context, Arc::clone(&conn), req);
                }
                Err(WsError::Protocol(ProtocolError::ResetWithoutClosingHandshake)) => {
                    info!(?addr, "Receive side terminated");
                    break;
                }
                Err(err) => {
                    error!(?addr, "Error while receiving {:?}", err);
                    break;
                }
            }
        }
        states.connection.remove(&context.connection_id);
        info!(?addr, "Connection closed");
    }
    pub async fn send_msg<S: AsyncRead + AsyncWrite + Unpin + Send + 'static>(
        self: Arc<Self>,
        states: Arc<WebsocketStates<S>>,
        mut message_receiver: mpsc::Receiver<WsMessage>,
    ) {
        while let Some(msg) = message_receiver.recv().await {
            let conn = states.connection.get_mut(&msg.connection_id);
            if let Some(mut conn) = conn {
                let self1 = &msg.message;
                let result = conn
                    .ws_sink
                    .send(Message::Text(
                        serde_json::to_string(self1).expect("Failed to dump json(impossible)"),
                    ))
                    .await;
                if let Err(err) = result {
                    error!(?conn.conn.address, "Error while sending {:?}", err);
                }
            } else {
                error!(?msg.connection_id, "Connection not found");
            }
        }
    }
    pub async fn listen(self) -> Result<()> {
        if self.config.pub_cert.is_empty() && self.config.priv_cert.is_empty() {
            self.listen_tcp().await
        } else if !self.config.pub_cert.is_empty() && !self.config.priv_cert.is_empty() {
            self.listen_tls().await
        } else {
            bail!("pub_cert and priv_cert should be both set or unset")
        }
    }
    async fn listen_tcp(mut self) -> Result<()> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        info!("{} listening on {}(tcp)", self.config.name, addr);

        let message_receiver = self.message_receiver.take().unwrap();
        let this = Arc::new(self);
        let states = Arc::new(WebsocketStates::new());
        tokio::spawn(Arc::clone(&this).send_msg(Arc::clone(&states), message_receiver));
        let listener = tokio::net::TcpListener::bind(addr).await?;
        loop {
            let (stream, addr) = listener.accept().await?;

            info!("Accepted stream from {}", addr);
            tokio::spawn(Arc::clone(&this).handle_request(addr, Arc::clone(&states), stream));
        }
    }
    async fn listen_tls(mut self) -> Result<()> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        info!("{} listening on {}(tls)", self.config.name, addr);
        // Build TLS configuration.
        let tls_cfg = {
            // Load public certificate.
            let certs = load_certs(&self.config.pub_cert)?;
            // Load private key.
            let key = load_private_key(&self.config.priv_cert)?;
            // Do not use client certificate authentication.
            let mut cfg = rustls::ServerConfig::builder()
                .with_safe_defaults()
                .with_no_client_auth()
                .with_single_cert(certs, key)?;
            // Configure ALPN to accept HTTP/2, HTTP/1.1 in that order.
            cfg.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
            Arc::new(cfg)
        };
        let message_receiver = self.message_receiver.take().unwrap();
        let this = Arc::new(self);
        let states = Arc::new(WebsocketStates::new());
        tokio::spawn(Arc::clone(&this).send_msg(Arc::clone(&states), message_receiver));
        let listener = tokio::net::TcpListener::bind(addr).await?;
        let acceptor = TlsAcceptor::from(tls_cfg);
        loop {
            let (stream, addr) = listener.accept().await?;
            let stream = acceptor.accept(stream).await?;

            info!("Accepted stream from {}", addr);
            tokio::spawn(Arc::clone(&this).handle_request(addr, Arc::clone(&states), stream));
        }
    }
}
// Load public certificate from file.
fn load_certs(filename: &str) -> Result<Vec<rustls::Certificate>> {
    // Open certificate file.
    let certfile = File::open(filename).map_err(|e| eyre!("failed to open {}: {}", filename, e))?;
    let mut reader = BufReader::new(certfile);

    // Load and return certificate.
    let _certs =
        rustls_pemfile::certs(&mut reader).map_err(|_| eyre!("failed to load certificate"))?;
    
    let pub_key = parse(fs::read(filename).unwrap()).unwrap().contents;

    Ok(vec!(rustls::Certificate(pub_key)))
    
    //Ok(certs.into_iter().map(rustls::Certificate).collect())
}

// Load private key from file.
fn load_private_key(filename: &str) -> Result<rustls::PrivateKey> {
    // Open keyfile.
    let keyfile = File::open(filename).map_err(|e| eyre!("failed to open {}: {}", filename, e))?;
    
    let mut reader = BufReader::new(keyfile);

    // Load and return a single private key.
    let _keys = rustls_pemfile::rsa_private_keys(&mut reader)
        .map_err(|_| eyre!("failed to load private key"))?;
    

    let priv_key = parse(fs::read(filename).unwrap()).unwrap().contents;

    //test_rustls_pem();

    if priv_key.len() == 0 {
        bail!("expected a single private key");
    }

    Ok(rustls::PrivateKey(priv_key))
}

fn test_rustls_pem(){
    let filename="./key.pem";
    // Open keyfile.
    let keyfile = File::open(filename).map_err(|e| eyre!("failed to open {}: {}", filename, e)).unwrap();
    
    let mut reader = BufReader::new(keyfile);

    // Load and return a single private key.
    let keys = rustls_pemfile::rsa_private_keys(&mut reader)
        .map_err(|_| eyre!("failed to load private key"));
    println!("keys in test_rustls={:?}",keys);

    let priv_key = parse(fs::read("key.pem").unwrap()).unwrap().contents;
    
    println!("priv_key: {:?}", priv_key.len()); // 48    
    // if keys.len() != 1 {
    //     println!("expected a single private key");
    // }
}
