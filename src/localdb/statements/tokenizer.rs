use itertools::Itertools;

pub fn tokenize_statements(statements: &str, tokens: Vec<String>) -> Result<String,String> {
	let (num_placeholders, placeholders) = unique_placeholders(statements)?;

	if num_placeholders != tokens.len() {
		return Err("mismatched number of unique placeholders and tokens".to_owned());
	};

	if tokens.len() == 0 {
		return Ok(statements.to_owned());
	};
	
	let mut placeholder_to_token = placeholders
		.into_iter()
		.zip(tokens)
		.collect::<Vec<_>>();

	placeholder_to_token
		.sort_by(
			|a, b|(
				b.0).clone().pop().cmp(&(a.0).clone().pop()
			)
	);

	let mut tokenized_statements: String = statements.to_owned();
	for (placeholder, token) in placeholder_to_token {
		tokenized_statements = tokenized_statements.replace(
			&placeholder,
			format_token(token.to_owned()).as_str(),
		);
	};
	
	Ok(tokenized_statements)
}

fn unique_placeholders(statements: &str) -> Result<(usize, Vec<String>), String> {
	let token_placeholders = regex::bytes::Regex::new(r#"(?:\?[0-9]+)"#).unwrap();
	let captured_matches = token_placeholders.captures_iter(statements.as_bytes());

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
	let number = regex::bytes::Regex::new(r#"^[0-9]+\.?[0-9]*$"#).unwrap();
	if number.is_match(token.as_bytes()) {
		// if string is numeric, use string
		token
	} else {
		// if it's not, add quotes
		format!("{}{}{}","\"",token,"\"")
	}
}
