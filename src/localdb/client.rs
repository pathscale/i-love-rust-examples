use serde_json;

use super::database;

pub struct Client {
	db: database::Database,
	checker: database::QueryChecker,
}

impl Client {
	pub fn new(path: &str) -> Self {
		Self{
			db: database::Database::new(path),
			checker: database::QueryChecker::default(),
		}
	}

	pub fn write(&mut self, query: &str) -> Result<(),String> {
		let valid_query = self.checker.is_read(query);
		let read_only = match valid_query {
			Ok(is_read_only) => is_read_only,
			Err(error) => return Err(error.to_string()),
		};
		if read_only {
			return Err("can't call write with a read only statement".to_owned())
		};

		match self.db.query(query) {
			Ok(_) => Ok(()),
			Err(error) => return Err(error.to_string()),
		}
	}


	pub fn read(&mut self, query: &str) -> Result<String,String> {
		let valid_query = self.checker.is_read(query);
		let read_only = match valid_query {
			Ok(is_read_only) => is_read_only,
			Err(error) => return Err(error.to_string()),
		};
		if !read_only {
			return Err("can't call write with a read only statement".to_owned())
		};

		let serialized = match self.db.query(query) {
			Ok(result) => serde_json::to_string(&result),
			Err(error) => return Err(error.to_string()),
		};

		match serialized {
			Ok(json) => Ok(json),
			Err(error) => Err(error.to_string()),
		}
	}
}

impl Default for Client {
	fn default() -> Self {
		Self{
			db: database::Database::default(),
			checker: database::QueryChecker::default(),
		}
	}
}
