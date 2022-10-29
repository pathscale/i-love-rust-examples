use deadpool::managed;

use super::connection::{DbConnection, DbConnectionError};

pub struct LocalDbConnectionManager {
    config: crate::config::DbConfig,
}

impl LocalDbConnectionManager {
    pub fn new(config: crate::config::DbConfig) -> Self {
        Self { config: config }
    }
}

#[deadpool::async_trait]
impl managed::Manager for LocalDbConnectionManager {
    type Type = DbConnection;
    type Error = DbConnectionError;

    async fn create(&self) -> Result<DbConnection, DbConnectionError> {
        Ok(DbConnection::new(&self.config.host, &self.config.port).await?)
    }

    async fn recycle(&self, conn: &mut DbConnection) -> managed::RecycleResult<DbConnectionError> {
        conn.close("recycled").await?;
        conn.open().await?;
        Ok(())
    }
}
