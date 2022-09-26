use serde_json;

use super::database;
use super::statements::{tokenizer,analyzer};

pub struct Client {
	db: database::Database,
}

impl Client {
	pub fn new(storage_path: &str) -> Self {
		Self{
			db: database::Database::new(storage_path),
		}
	}

	pub fn write(&mut self, statements: &str, tokens: Vec<String>) -> Result<(),String> {
		let tokenized_statements = tokenizer::tokenize_statements(statements, tokens)?;
		let valid_statements = analyzer::are_read_only(&tokenized_statements);
		let read_only = match valid_statements {
			Ok(is_read_only) => is_read_only,
			Err(error) => return Err(error.to_string()),
		};
		if read_only {
			return Err("can't call write solely with read-only statements".to_owned())
		};

		match self.db.exec(&tokenized_statements) {
			Ok(_) => Ok(()),
			Err(error) => return Err(error.to_string()),
		}
	}


	pub fn read(&mut self, statements: &str, tokens: Vec<String>) -> Result<String,String> {
		let tokenized_statements = tokenizer::tokenize_statements(statements, tokens)?;
		let valid_statements = analyzer::are_read_only(&tokenized_statements);
		let read_only = match valid_statements {
			Ok(is_read_only) => is_read_only,
			Err(error) => return Err(error.to_string()),
		};
		if !read_only {
			return Err("can't call read with non read-only statements".to_owned())
		};

		let serialized = match self.db.exec(&tokenized_statements) {
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
		}
	}
}
