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

#[derive(Debug)]
pub enum ConnectError {
    BuildError(deadpool::managed::BuildError<DbConnectionError>),
    Message(&'static str),
}

impl std::fmt::Display for ConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BuildError(e) => write!(f, "{:?}", e),
            Self::Message(error_msg) => write!(f, "{:?}", error_msg),
        }
    }
}

impl std::error::Error for ConnectError {}

impl From<deadpool::managed::BuildError<DbConnectionError>> for ConnectError {
    fn from(e: deadpool::managed::BuildError<DbConnectionError>) -> Self {
        Self::BuildError(e)
    }
}

impl From<&'static str> for ConnectError {
    fn from(e: &'static str) -> Self {
        Self::Message(e)
    }
}
