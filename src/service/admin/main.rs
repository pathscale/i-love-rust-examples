mod method;

use crate::endpoints::{endpoint_admin_assign_role, endpoint_admin_list_users};
use crate::method::{AssignRoleHandler, ListUsersHandler};
use eyre::*;
use gen::model::EnumService;
use iloverust::endpoints::endpoint_auth_authorize;
use iloverust::method::AuthorizeHandler;
use lib::config::load_config;
use lib::database::connect_to_database;
use lib::log::setup_logs;
use lib::ws::{EndpointAuthController, WebsocketServer};
use std::sync::Arc;

pub mod endpoints;

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config("admin".to_owned())?;
    setup_logs(config.app.log_level)?;

    let db = connect_to_database(config.db).await?;
    let mut server = WebsocketServer::new(config.app);
    server.add_database(db);
    let auth_controller = Arc::new(EndpointAuthController::new(server.get_toolbox()));
    auth_controller.add_auth_endpoint(
        endpoint_auth_authorize(),
        AuthorizeHandler {
            accept_service: EnumService::Admin,
        },
    );
    server.add_auth_controller(auth_controller);
    server.add_handler(endpoint_admin_list_users(), ListUsersHandler);
    server.add_handler(endpoint_admin_assign_role(), AssignRoleHandler);
    server.listen().await?;
    Ok(())
}
