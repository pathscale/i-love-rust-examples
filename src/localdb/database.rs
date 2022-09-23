use crate::localdb::client;
use crate::localdb::constants;

pub struct Database {
	client: client::Client
}

impl Database {
	pub fn new(path: &str) -> Self {
		let mut new = Self{client: client::Client::new(path)};
		new.init();
		new
	}

	fn init(&mut self) {
		for create_table_query in constants::TABLES {
			let output = self.client.query(create_table_query);
			println!("{:?}", output);
		}
		for create_index_query in constants::INDEXES {
			let output = self.client.query(create_index_query);
			println!("{:?}", output);
		}
	}
}

impl Default for Database {
	fn default() -> Self {
		let mut default = Self{client: client::Client::default()};
		default.init();
		default
	}
}
