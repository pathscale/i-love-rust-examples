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

pub mod endpoints {
    use super::*;
    include!("endpoints.rs");
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config("auth".to_owned())?;
    setup_logs(config.app.log_level)?;

    let db = connect_to_database(config.db).await?;
    let mut server = WebsocketServer::new(config.app);
    server.add_database(db);
    server.add_handler(endpoint_auth_signup(), SignupHandler);
    server.add_handler(endpoint_auth_login(), LoginHandler);
    server.listen().await?;
    Ok(())
}
