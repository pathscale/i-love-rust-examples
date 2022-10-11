use std::sync::Arc;

use eyre::Result;
use futures::lock::Mutex;

use lib::config::load_config;
use lib::log::setup_logs;
use lib::ws::WebsocketServer;

use localdb::db::database::Database;
use localdb::endpoints::endpoint_localdb_select;
use localdb::method::QueryHandler;

#[tokio::main]
async fn main() -> Result<()> {
    let db = Database::default();

    let config = load_config("localdb".to_owned())?;
    setup_logs(config.app.log_level)?;

    let mut server = WebsocketServer::new(config.app);
    server.add_handler(
        endpoint_localdb_select(),
        QueryHandler {
            db: Arc::new(Mutex::new(db)),
        },
    );
    server.listen().await?;
    Ok(())
}
