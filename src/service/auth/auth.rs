mod method;

use crate::endpoints::{endpoint_auth_login, endpoint_auth_signup};
use crate::method::{LoginHandler, SignupHandler};
use eyre::*;
use lib::config::load_config;
use lib::database::connect_to_database;
use lib::log::setup_logs;
use lib::ws::WebsocketServer;
use model::endpoint::*;
use serde::*;
use tracing::*;

pub mod endpoints {
    use super::*;
    include!("endpoints.rs");
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config("auth".to_owned())?;
    println!("Log level: {:?}", config.app.log_level);
    setup_logs(config.app.log_level)?;

    info!(
        "Connecting to database {}:{}",
        config.db.host.as_ref().unwrap(),
        config.db.port.as_ref().unwrap()
    );
    let db = connect_to_database(config.db).await?;
    info!("Starting {} server", config.app.name);
    let mut server = WebsocketServer::new();
    server.add_database(db);
    server.add_handler(endpoint_auth_signup(), SignupHandler);
    server.add_handler(endpoint_auth_login(), LoginHandler);
    server
        .listen(&format!("0.0.0.0:{}", config.app.port))
        .await?;
    Ok(())
}
