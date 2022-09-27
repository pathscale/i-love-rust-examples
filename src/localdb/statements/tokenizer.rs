use itertools::Itertools;

pub fn tokenize_statements(statements: &str, tokens: Vec<String>) -> Result<String,String> {
	let (num_placeholders,mut placeholders) = unique_placeholders(statements)?;
	order_placeholders(&mut placeholders);
	validate_tokenization(num_placeholders, tokens.len(), &placeholders)?;

	if tokens.len() == 0 { return Ok(statements.to_owned()) };
	
	let mut tokenized_statements: String = statements.to_owned();
	for (idx, placeholder) in placeholders.iter().enumerate().rev() {
		tokenized_statements = tokenized_statements.replace(
			placeholder,
			&format_token(tokens[idx].to_owned()),
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

fn order_placeholders(placeholders: &mut Vec<String>) {
	placeholders.sort_by(|a,b| a.clone().pop().cmp(&b.clone().pop()));
}

fn validate_tokenization(num_placeholders: usize, num_tokens: usize, placeholders: &Vec<String>) -> Result<(),String> {
	if num_placeholders != num_tokens {
		return Err("mismatched number of unique placeholders and tokens".to_owned());
	};

	for (idx, placeholder) in placeholders.iter().enumerate() {
		if placeholder != &format!("?{}",&idx.to_string()) {
			return Err("mismatched placeholder numbers and token vector indexes".to_owned());
		}
	}

	Ok(())
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
