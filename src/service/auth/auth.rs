mod method;

use crate::method::HandleAuthFoo;
use lib::logger::setup_logs;
use lib::model::*;
use lib::ws::WebsocketHandler;
use std::sync::Arc;
use tracing::level_filters::LevelFilter;
use tracing::*;

include!("../../gen/database.rs");
include!("endpoints.rs");

#[tokio::main]
async fn main() -> Result<()> {
    setup_logs(LevelFilter::DEBUG)?;
    info!("Starting exchange_auth server");
    let mut executor = WebsocketHandler::new();
    executor.add_handler_raw(endpoint_auth_foo(), Arc::new(HandleAuthFoo));
    Arc::new(executor).listen("0.0.0.0:4444").await?;
    Ok(())
}
