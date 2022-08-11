use lib::logger::setup_logs;
use lib::ws::{JsonVerifier, WebsocketHandler};
use std::sync::Arc;
use tracing::level_filters::LevelFilter;
use tracing::*;
include!("../../gen/database.rs");

#[tokio::main]
async fn main() -> Result<()> {
    setup_logs(LevelFilter::DEBUG)?;
    info!("Starting exchange_auth server");
    let executor = WebsocketHandler {
        handlers: Default::default(),
        connection: Default::default(),
        verifier: JsonVerifier {},
    };
    Arc::new(executor).listen("0.0.0.0:4444").await?;
    Ok(())
}
