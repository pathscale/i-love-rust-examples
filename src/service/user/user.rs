mod method;

use crate::endpoints::endpoint_user_foo;
use crate::method::FooHandler;
use eyre::*;
use lib::config::load_config;
use lib::database::connect_to_database;
use lib::log::setup_logs;
use lib::ws::WebsocketServer;
use model::endpoint::*;
use serde::*;
use std::sync::Arc;
use tracing::*;

pub mod endpoints {
    use super::*;
    include!("endpoints.rs");
}
include!("../auth/header.rs");

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config("user".to_owned())?;
    setup_logs(config.app.log_level)?;

    let db = connect_to_database(config.db).await?;
    let mut server = WebsocketServer::new(config.app);
    server.add_auth_controller(Arc::new(LoginAuthController {
        db: db.clone().into(),
    }));
    server.add_database(db);
    server.add_handler(endpoint_user_foo(), FooHandler);
    server.listen().await?;
    Ok(())
}
