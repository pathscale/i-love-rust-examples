use gluesql::core::{result, sqlparser, parse_sql};
use sqlparser::ast::Statement;

pub fn are_read_only(statements: &str) -> result::Result<bool> {
	let statements_vector = get_statement_types(statements)?;
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


fn get_statement_types(statements: &str) -> result::Result<Vec<sqlparser::ast::Statement>> {
	parse_sql::parse(statements)
}
