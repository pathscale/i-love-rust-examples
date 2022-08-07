use tokio::sync::mpsc;
use eyre::*;
use async_compat::CompatExt;
use futures::Sink;
use tracing::level_filters::LevelFilter;
use tracing::*;
use pin_utils::pin_mut;
use async_tungstenite::tungstenite::handshake::server::{Callback, Request, Response, ErrorResponse};
use rust_example::analyze::{thread2, thread3};
use rust_example::logger::setup_logs;
use rust_example::poll_data::fetch_from_polygon;
use rust_example::utils::{error_handled, error_handled_sync};
use rust_example::YourDataStruct;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logs(LevelFilter::DEBUG)?;
    info!("Starting exchange_auth server");
    websocket_handler().await?;
    Ok(())
}

