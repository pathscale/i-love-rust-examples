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

	pub fn write(&mut self, query: &str, tokens: Vec<String>) -> Result<(),String> {
		let tokenized_query = tokenize_query(query, tokens)?;
		let valid_query = self.checker.is_read(&tokenized_query);
		let read_only = match valid_query {
			Ok(is_read_only) => is_read_only,
			Err(error) => return Err(error.to_string()),
		};
		if read_only {
			return Err("can't call write with a read only statement".to_owned())
		};

		match self.db.query(&tokenized_query) {
			Ok(_) => Ok(()),
			Err(error) => return Err(error.to_string()),
		}
	}


	pub fn read(&mut self, query: &str, tokens: Vec<String>) -> Result<String,String> {
		let tokenized_query = tokenize_query(query, tokens)?;
		let valid_query = self.checker.is_read(&tokenized_query);
		let read_only = match valid_query {
			Ok(is_read_only) => is_read_only,
			Err(error) => return Err(error.to_string()),
		};
		if !read_only {
			return Err("can't call write with a read only statement".to_owned())
		};

		let serialized = match self.db.query(&tokenized_query) {
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

fn tokenize_query(query: &str, tokens: Vec<String>) -> Result<String,String> {
	let token_chars = regex::bytes::Regex::new(r#"\?"#).unwrap();
	if token_chars.find_iter(query.as_bytes()).count() != tokens.len() {
		return Err("mismatched tokens and string vector length".to_owned());
	};

	if tokens.len() == 0 {
		return Ok(query.to_owned());
	};
	
	let mut tokenized_query: String = query.to_owned();
	for token in tokens {
		tokenized_query = tokenized_query.replacen("?", format_token(token).as_str(), 1);
	};

	Ok(tokenized_query)
}

fn format_token(token: String) -> String {
	let numeric = regex::bytes::Regex::new(r#"^[0-9]*$"#).unwrap();
	if numeric.is_match(token.as_bytes()) {
		// if string is numeric, use string
		token
	} else {
		// if it's not, add quotes
		format!("{}{}{}","\"",token,"\"")
	}
}
