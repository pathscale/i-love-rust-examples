use gluesql::prelude::{Glue, SledStorage};
use gluesql::core::{executor};

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
			let output = self.query(create_table_query);
			println!("{:?}", output);
		}
		for create_index_query in sql::create::INDEXES {
			let output = self.query(create_index_query);
			println!("{:?}", output);
		}
	}

	pub fn query(&mut self, query: &str) -> Vec<executor::Payload> {
		self.inner.execute(query).unwrap()
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
