use deadpool::managed;
use eyre::*;
use gluesql::core::executor::Payload;
use serde::Deserialize;
use serde::Serialize;
use serde_json;

use super::connection::DbConnectionError;
use super::manager::LocalDbConnectionManager;
use super::stringable::Stringable;

#[derive(Debug, Serialize)]
pub struct DbRequest {
    method: u32,
    seq: u32,
    params: ReqParams,
}

#[derive(Debug, Serialize)]
pub struct ReqParams {
    statements: String,
    tokens: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct DbResponse {
    method: u32,
    seq: u32,
    params: ResParams,
}

#[derive(Debug, Deserialize)]
pub struct ResParams {
    payloads: Vec<Payload>,
}

#[derive(Clone)]
pub struct LocalDbClient {
    pool: managed::Pool<LocalDbConnectionManager>,
}

impl LocalDbClient {
    pub fn new(pool: managed::Pool<LocalDbConnectionManager>) -> Self {
        Self { pool: pool }
    }

    pub async fn query<S>(
        &self,
        statements: &S,
        tokens: &[&(dyn Stringable + Sync)],
    ) -> Result<Vec<Payload>, LocalDbClientError>
    where
        S: ?Sized + Sync + Stringable,
    {
        let params = parse_req_params(statements, tokens);
        let req = DbRequest {
            method: 40010,
            seq: 0,
            params: params,
        };

        let mut conn = self.pool.get().await?;

        let req_string = serde_json::to_string(&req)?;
        conn.write(req_string).await?;

        let res = serde_json::from_str::<DbResponse>(&conn.read().await?)?;

        Ok(res.params.payloads)
    }
}

fn parse_req_params<S>(
    statements: &S,
    tokens: &[&(dyn Stringable + Sync)],
) -> ReqParams
where
    S: ?Sized + Sync + Stringable,
{
    let parsed_statements = statements.stringify();
    let parsed_tokens = tokens
        .clone()
        .to_owned()
        .iter_mut()
        .map(|t| t.stringify())
        .collect();

		ReqParams {
				statements: parsed_statements,
				tokens: parsed_tokens,
		}
}

#[derive(Debug)]
pub enum LocalDbClientError {
    DeserializationError(serde_json::Error),
    PoolError(deadpool::managed::PoolError<DbConnectionError>),
    BuildError(deadpool::managed::BuildError<ErrReport>),
    DbConnectionError(DbConnectionError),
    Message(&'static str),
}

impl std::fmt::Display for LocalDbClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeserializationError(e) => write!(f, "{:?}", e),
            Self::PoolError(e) => write!(f, "{:?}", e),
            Self::BuildError(e) => write!(f, "{:?}", e),
            Self::DbConnectionError(e) => write!(f, "{:?}", e),
            Self::Message(error_msg) => write!(f, "{:?}", error_msg),
        }
    }
}

impl std::error::Error for LocalDbClientError {}

impl From<serde_json::Error> for LocalDbClientError {
    fn from(e: serde_json::Error) -> Self {
        Self::DeserializationError(e)
    }
}

impl From<deadpool::managed::PoolError<DbConnectionError>> for LocalDbClientError {
    fn from(e: deadpool::managed::PoolError<DbConnectionError>) -> Self {
        Self::PoolError(e)
    }
}

impl From<DbConnectionError> for LocalDbClientError {
    fn from(e: DbConnectionError) -> Self {
        Self::DbConnectionError(e)
    }
}

impl From<&'static str> for LocalDbClientError {
    fn from(e: &'static str) -> Self {
        Self::Message(e)
    }
}
