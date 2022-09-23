use gluesql::prelude::{Glue, SledStorage};
use gluesql::core::{executor, result};

use super::sql;

pub struct Database {
	inner: Glue<SledStorage>,
}

impl Database {
	pub fn new(path: &str) -> Self {
		let storage = SledStorage::new(path).unwrap();		
		let mut db = Self{inner: Glue::new(storage)};
		db.init();
		db
	}

	fn init(&mut self) {
		for create_table_query in sql::create::TABLES {
			match self.query(create_table_query) {
				Ok(output) => println!("{:?}", output),
				Err(error) => println!("{:?}", error),
			};
		}
		for create_index_query in sql::create::INDEXES {
			// BUG: gluesql will throw error when creating index in case it already exists
			// even if "IF NOT EXISTS" is used
			// TODO: update library when bug is fixed
			match self.query(create_index_query) {
				Ok(output) => println!("{:?}", output),
				Err(error) => println!("{:?}", error),
			};
		}
	}

	pub fn query(&mut self, query: &str) -> result::Result<Vec<executor::Payload>> {
		self.inner.execute(query)
	}
}

impl Default for Database {
	fn default() -> Self {
		let storage = SledStorage::new("storage").unwrap();		
		let mut db = Self{inner: Glue::new(storage)};
		db.init();
		db
	}
}
