use gluesql::core::{result, sqlparser, parse_sql};
use sqlparser::ast::Statement;

pub fn are_read_only(statements: &str) -> Result<bool,AnalyzerError> {
	let statements_vector = parse_sql::parse(statements)?;
	for statement in statements_vector {
		match statement {
			// read/write agnostic statements
			Statement::Comment{..} => ||{},
			Statement::Declare{..} => ||{},
			Statement::SetVariable{..} => ||{},
			Statement::Prepare{..} => ||{},
			Statement::Execute{..} => ||{},
			Statement::Deallocate{..} => ||{},
			Statement::Kill{..} => ||{},
			Statement::Assert{..} => ||{},
			Statement::Fetch{..} => ||{},
			Statement::Close{..} => ||{},
			Statement::Discard{..} => ||{},
			// read-only statements
			Statement::Query(..) => ||{},
			Statement::ShowVariable{..} => ||{},
			Statement::Explain{..} => ||{},
			_ => return Ok(false),
		};
	}
	Ok(true)
}

#[derive(Debug)]
pub enum AnalyzerError {
	StatementParseError(result::Error),
}

impl From<result::Error> for AnalyzerError {
	fn from(e: result::Error) -> Self {
		Self::StatementParseError(e)
	}
}
