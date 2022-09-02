use deadpool_postgres::Runtime;
use deadpool_postgres::*;
use eyre::*;
use tokio_postgres::types::ToSql;
use tokio_postgres::{NoTls, Row, ToStatement};

pub type DatabaseConfig = deadpool_postgres::Config;
#[derive(Clone)]
pub struct SimpleDbClient {
    pool: Pool,
}
impl SimpleDbClient {
    pub async fn query<T>(
        &self,
        statement: &T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Row>, Error>
    where
        T: ?Sized + ToStatement,
    {
        Ok(self.pool.get().await?.query(statement, params).await?)
    }
}

pub async fn connect_to_database(config: DatabaseConfig) -> Result<SimpleDbClient> {
    let pool = config.create_pool(Some(Runtime::Tokio1), NoTls)?;
    Ok(SimpleDbClient { pool })
}
