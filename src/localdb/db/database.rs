use gluesql::core::executor::Payload;
use gluesql::prelude::{Glue, SledStorage};
use thiserror::Error;

pub struct Database {
    inner: Glue<SledStorage>,
}

impl Database {
    pub fn new(storage_path: &str) -> Result<Self, DatabaseError> {
        let storage =
            SledStorage::new(storage_path).or_else(|_| Err(DatabaseError::InstantiationError))?;
        let mut db = Self {
            inner: Glue::new(storage),
        };
        db.init()?;
        Ok(db)
    }

    fn init(&mut self) -> Result<(), DatabaseError> {
        for create_statement in include_str!("../../../db/tbl.sql").split_terminator(";") {
            // BUG: gluesql will throw error when creating index in case it already exists
            // even if "IF NOT EXISTS" is used
            // TODO: update library when bug is fixed
            match self.exec(create_statement) {
                Ok(_) => (),
                Err(error) => println!("{:?}", error),
            };
        }
        Ok(())
    }

    pub fn exec(&mut self, statements: &str) -> Result<Vec<Payload>, DatabaseError> {
        Ok(self.inner.execute(statements)?)
    }
}

impl Default for Database {
    fn default() -> Self {
        let storage = SledStorage::new("storage")
            .unwrap_or_else(|e| panic!("could not instantiate database: {:?}", e));
        let mut db = Self {
            inner: Glue::new(storage),
        };
        db.init()
            .unwrap_or_else(|e| panic!("could not create database: {:?}", e));
        db
    }
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("query execution failed")]
    ExecError(#[from] gluesql::core::result::Error),
    #[error("database instantiation failed")]
    InstantiationError,
}
