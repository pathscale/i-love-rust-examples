use std::sync::Arc;

use eyre::*;


use tracing::level_filters::LevelFilter;
use tracing::*;



use rust_example::logger::setup_logs;


use rust_example::ws::WebsocketHandler;


#[tokio::main]
async fn main() -> Result<()> {
    setup_logs(LevelFilter::DEBUG)?;
    info!("Starting exchange_auth server");
    let executor = WebsocketHandler {
        handlers: Default::default(),
        connection: Default::default()
    };
    Arc::new(executor).listen().await?;
    Ok(())
}

