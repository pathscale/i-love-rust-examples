use serde_json;
use itertools::Itertools;

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
	let (num_placeholders, placeholders) = unique_placeholders(query)?;

	if num_placeholders != tokens.len() {
		return Err("mismatched placeholders and token vector length".to_owned());
	};

	if tokens.len() == 0 {
		return Ok(query.to_owned());
	};
	
	let mut placeholders_and_tokens = placeholders.into_iter().zip(tokens).collect::<Vec<_>>();
	placeholders_and_tokens.sort_by(|a, b|(b.0).clone().pop().cmp(&(a.0).clone().pop()));

	let mut tokenized_query: String = query.to_owned();
	for (placeholder, token) in placeholders_and_tokens {
		tokenized_query = tokenized_query.replace(
			&placeholder,
			format_token(token.to_owned()).as_str()
		);
	};
	
	Ok(tokenized_query)
}

fn unique_placeholders(query: &str) -> Result<(usize, Vec<String>), String> {
	let token_placeholders = regex::bytes::Regex::new(r#"(?:\?[0-9]+)"#).unwrap();
	let captured_matches = token_placeholders.captures_iter(query.as_bytes());

	let mut matches: Vec<&[u8]> = Vec::new();
	let mut placeholders: Vec<String> = Vec::new();
	for capture in captured_matches {
		let match_bytes =	match capture.get(0) {
				None => return Err("could not recover captured placeholders".to_owned()),
				Some(c) => c.as_bytes(),
		};
		matches.push(match_bytes);
		let placeholder = match std::str::from_utf8(match_bytes) {
			Err(error) => return Err(error.to_string()),
			Ok(p) => p,
		};
		placeholders.push(placeholder.to_owned());
	}

	Ok((matches.iter().unique().count(), placeholders.into_iter().unique().collect()))
}

fn format_token(token: String) -> String {
	let number = regex::bytes::Regex::new(r#"^[0-9]*\.?[0-9]*$"#).unwrap();
	if number.is_match(token.as_bytes()) {
		// if string is numeric, use string
		token
	} else {
		// if it's not, add quotes
		format!("{}{}{}","\"",token,"\"")
	}
}
