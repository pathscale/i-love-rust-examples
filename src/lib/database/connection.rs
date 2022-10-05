use futures::stream::FusedStream;
use futures::SinkExt;
use futures::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::{CloseFrame, Message};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub struct DbConnection {
    inner: WebSocketStream<MaybeTlsStream<TcpStream>>,
    host: String,
    port: u32,
    pending: bool,
}

impl DbConnection {
    pub async fn new(host: &str, port: &u32) -> Result<Self, DbConnectionError> {
        Ok(Self {
            inner: connect_async(format!("ws://{}:{}", host, port)).await?.0,
            host: host.to_owned(),
            port: port.to_owned(),
            pending: false,
        })
    }

    pub fn is_open(&self) -> bool {
        !(self.inner.is_terminated())
    }

    pub fn is_pending(&self) -> bool {
        self.pending
    }

    pub async fn write(&mut self, msg: String) -> Result<(), DbConnectionError> {
        self.pending = true;
        self.inner.send(Message::text(msg)).await?;
        self.pending = false;
        Ok(())
    }

    pub async fn read(&mut self) -> Result<String, DbConnectionError> {
        self.pending = true;
        let message = match self.inner.next().await {
            Some(r) => r?,
            None => return Err("no response".into()),
        };
        self.pending = false;
        Ok(message.into_text()?)
    }

    pub async fn open(&mut self) -> Result<(), DbConnectionError> {
        if self.is_open() {
            return Err("connection already open".into());
        };
        self.inner = connect_async(format!("ws://{}:{}", self.host, self.port))
            .await?
            .0;
        Ok(())
    }

    pub async fn close(&mut self, reason: &str) -> Result<(), DbConnectionError> {
        if !self.is_open() {
            return Err("connection already closed".into());
        };
        if self.is_pending() {
            return Err("cannot close connection with pending data".into());
        };
        self.inner
            .close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: std::borrow::Cow::from(reason),
            }))
            .await?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum DbConnectionError {
    WebsocketError(tokio_tungstenite::tungstenite::Error),
    Message(&'static str),
}

impl std::fmt::Display for DbConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WebsocketError(e) => write!(f, "{:?}", e),
            Self::Message(error_msg) => write!(f, "{:?}", error_msg),
        }
    }
}

impl std::error::Error for DbConnectionError {}

impl From<tokio_tungstenite::tungstenite::Error> for DbConnectionError {
    fn from(e: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::WebsocketError(e)
    }
}

impl From<&'static str> for DbConnectionError {
    fn from(e: &'static str) -> Self {
        Self::Message(e)
    }
}
