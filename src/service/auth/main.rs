mod method;

use crate::endpoints::{endpoint_auth_login, endpoint_auth_signup};
use crate::method::{LoginHandler, SignupHandler};
use eyre::*;
use lib::config::load_config;
use lib::database::connect_to_database;
use lib::log::setup_logs;
use lib::ws::{EndpointAuthController, WebsocketServer};
use std::sync::Arc;

pub mod endpoints;

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config("auth".to_owned())?;
    setup_logs(config.app.log_level)?;

    let db = connect_to_database(config.db).await?;
    let mut server = WebsocketServer::new(config.app);
    server.add_database(db);
    let auth_controller = Arc::new(EndpointAuthController::new(server.get_toolbox()));
    auth_controller.add_auth_endpoint(endpoint_auth_login(), LoginHandler);
    auth_controller.add_auth_endpoint(endpoint_auth_signup(), SignupHandler);
    server.add_auth_controller(auth_controller.clone());
    server.listen().await?;
    Ok(())
}
