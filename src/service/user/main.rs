use crate::endpoints::endpoint_user_foo;
use crate::method::FooHandler;
use eyre::*;
use gen::model::EnumService;
use iloverust::endpoints::endpoint_auth_authorize;
use iloverust::method::AuthorizeHandler;
use lib::config::load_config;
use lib::database::connect_to_database;
use lib::id_gen::ConcurrentSnowflake;
use lib::log::setup_logs;
use lib::ws::{EndpointAuthController, WebsocketServer};
use std::sync::Arc;

pub mod endpoints;
pub mod method;

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config("user".to_owned())?;
    setup_logs(config.app.log_level)?;

    let db = connect_to_database(config.db).await?;
    let snowflake = ConcurrentSnowflake::new(EnumService::User as u16)?;

    let mut server = WebsocketServer::new(config.app);
    server.add_database(db);
    let auth_controller = Arc::new(EndpointAuthController::new(server.get_toolbox()));
    auth_controller.add_auth_endpoint(
        endpoint_auth_authorize(),
        AuthorizeHandler {
            accept_service: EnumService::User,
            id_gen: snowflake.clone(),
        },
    );
    server.add_auth_controller(auth_controller);
    server.add_handler(endpoint_user_foo(), FooHandler);
    server.listen().await?;
    Ok(())
}
