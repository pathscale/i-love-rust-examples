use gluesql::prelude::{Glue, SledStorage};
use gluesql::core::{result};

use super::sql;

pub struct Database {
	inner: Glue<SledStorage>,
}

pub type Payload = gluesql::core::executor::Payload;
pub type Label = Vec<String>;
pub type Row = Vec<gluesql::core::data::Value>;

impl Database {
	pub fn new(storage_path: &str) -> Result<Self,DatabaseError> {
		let storage = SledStorage::new(storage_path)
			.or_else(|_| Err(DatabaseError::from("could not instantiate database")))?;
		let mut db = Self{inner: Glue::new(storage)};
		db.init()?;
		Ok(db)
	}

	fn init(&mut self) -> Result<(),DatabaseError> {
		for create_table_statement in sql::create::TABLES {
			self.exec(create_table_statement)?;
		}
		for create_index_statement in sql::create::INDEXES {
			// BUG: gluesql will throw error when creating index in case it already exists
			// even if "IF NOT EXISTS" is used
			// TODO: update library when bug is fixed
			match self.exec(create_index_statement) {
				Ok(_) => (),
				Err(error) => println!("{:?}", error),
			};
		}
		Ok(())
	}

	pub fn exec(&mut self, statements: &str) -> Result<Vec<Payload>,DatabaseError> {
		Ok(self.inner.execute(statements)?)
	}
}

impl Default for Database {
	fn default() -> Self {
		let storage = SledStorage::new("storage")
			.unwrap_or_else(|e| panic!("could not instantiate database: {:?}", e));
		let mut db = Self{inner: Glue::new(storage)};
		db.init()
			.unwrap_or_else(|e| panic!("could not create database: {:?}", e));
		db
	}
}

#[derive(Debug)]
pub enum DatabaseError {
	ExecError(result::Error),
	Message(&'static str),
}

impl std::fmt::Display for DatabaseError {
	fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::ExecError(e) => write!(f, "{:?}", e),
			Self::Message(error_msg) => write!(f, "{:?}", error_msg),
		}
	}
}

impl std::error::Error for DatabaseError {}

impl From<result::Error> for DatabaseError {
	fn from(e: result::Error) -> Self {
		Self::ExecError(e)
	}
}

impl From<&'static str> for DatabaseError {
	fn from(e: &'static str) -> Self {
		Self::Message(e)
	}
}
