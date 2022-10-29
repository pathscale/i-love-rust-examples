use thiserror::Error;
use deadpool::managed::{Pool, PoolConfig, Timeouts};
use deadpool::Runtime;
use tracing::*;

use super::connection::DbConnectionError;
use super::manager::LocalDbConnectionManager;
use super::LocalDbClient;
use crate::config::DbConfig;

pub async fn connect_to_database(config: DbConfig) -> Result<LocalDbClient, ConnectError> {
    info!("Connecting to database {}:{}", config.host, config.port,);

    let manager = LocalDbConnectionManager::new(config);
    let pool = Pool::builder(manager)
        .runtime(Runtime::Tokio1)
        .config(PoolConfig::default())
        .timeouts(Timeouts::default())
        .build()?;

    Ok(LocalDbClient::new(pool))
}

#[derive(Debug, Error)]
pub enum ConnectError {
		#[error("connection pool builder failed")]
    BuildError(#[from] deadpool::managed::BuildError<DbConnectionError>),
}
