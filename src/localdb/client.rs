use serde_json;

use super::database;
use super::statements::{tokenizer,analyzer};

pub struct Client {
	db: database::Database,
}

impl Client {
	pub fn new(storage_path: &str) -> Result<Self,ClientError> {
		Ok(Self{ db: database::Database::new(storage_path)? })
	}

	pub fn write(&mut self, statements: &str, tokens: Vec<String>) -> Result<(),ClientError> {
		let tokenized_statements = tokenizer::tokenize_statements(statements, tokens)?;
		let read_only = analyzer::are_read_only(&tokenized_statements)?;

		if read_only {
			return Err(ClientError::from("can't call write solely with read-only statements"));
		};

		self.db.exec(&tokenized_statements)?;
		Ok(())
	}


	pub fn read(&mut self, statements: &str, tokens: Vec<String>) -> Result<String,ClientError> {
		let tokenized_statements = tokenizer::tokenize_statements(statements, tokens)?;
		let read_only = analyzer::are_read_only(&tokenized_statements)?;

		if !read_only {
			return Err(ClientError::from("can't call read with non read-only statements"));
		};

		Ok(serde_json::to_string(&self.db.exec(&tokenized_statements)?)?)
	}
}

impl Default for Client {
	fn default() -> Self {
		Self{
			db: database::Database::default(),
		}
	}
}

#[derive(Debug)]
pub enum ClientError {
	SerializationError(serde_json::Error),
	DbError(database::DatabaseError),
	ValidationError(analyzer::AnalyzerError),
	TokenizationError(tokenizer::TokenizerError),
	Message(&'static str),
}

impl From<serde_json::Error> for ClientError {
	fn from(e: serde_json::Error) -> Self {
		Self::SerializationError(e)
	}
}

impl From<database::DatabaseError> for ClientError {
	fn from(e: database::DatabaseError) -> Self {
		Self::DbError(e)
	}
}

impl From<analyzer::AnalyzerError> for ClientError {
	fn from(e: analyzer::AnalyzerError) -> Self {
		Self::ValidationError(e)
	}
}

impl From<tokenizer::TokenizerError> for ClientError {
	fn from(e: tokenizer::TokenizerError) -> Self {
		Self::TokenizationError(e)
	}
}

impl From<&'static str> for ClientError {
	fn from(e: &'static str) -> Self {
		Self::Message(e)
	}
}
