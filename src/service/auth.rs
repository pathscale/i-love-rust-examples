use std::sync::Arc;
use eyre::*;
use tracing::level_filters::LevelFilter;
use tracing::*;
use lib::logger::setup_logs;
use lib::ws::{JsonVerifier, WebsocketHandler};


#[tokio::main]
async fn main() -> Result<()> {
    setup_logs(LevelFilter::DEBUG)?;
    info!("Starting exchange_auth server");
    let executor = WebsocketHandler {
        handlers: Default::default(),
        connection: Default::default(),
        verifier: JsonVerifier {},
    };
    Arc::new(executor).listen().await?;
    Ok(())
}

