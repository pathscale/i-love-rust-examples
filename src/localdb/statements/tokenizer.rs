use itertools::Itertools;

pub fn tokenize_statements(statements: &str, tokens: Vec<String>) -> Result<String,TokenizerError> {
	let (num_placeholders,mut placeholders) = unique_placeholders(statements)?;
	order_placeholders(&mut placeholders);
	validate_matching_length(num_placeholders, tokens.len())?;
	validate_placeholders(&placeholders)?;

	if tokens.len() == 0 { return Ok(statements.to_owned()) };
	
	let mut tokenized_statements: String = statements.to_owned();
	for (idx, placeholder) in placeholders.iter().enumerate().rev() {
		tokenized_statements = tokenized_statements.replace(
			placeholder,
			&format_token(tokens[idx].to_owned())?,
		);
	};

	Ok(tokenized_statements)
}

fn unique_placeholders(statements: &str) -> Result<(usize, Vec<String>), TokenizerError> {
	let token_placeholders = regex::bytes::Regex::new(r#"(?:\?[0-9]+)"#)?;
	let captured_matches = token_placeholders.captures_iter(statements.as_bytes());

	let mut matches: Vec<&[u8]> = Vec::new();
	let mut placeholders: Vec<String> = Vec::new();
	for capture in captured_matches {
		let match_bytes =	capture.get(0)
			.ok_or("could not recover captured placeholders")?
			.as_bytes();
		matches.push(match_bytes);

		let placeholder = std::str::from_utf8(match_bytes)?;
		placeholders.push(placeholder.to_owned());
	}

	Ok((matches.iter().unique().count(), placeholders.into_iter().unique().collect()))
}

fn order_placeholders(placeholders: &mut Vec<String>) {
	placeholders.sort_by(|a,b| a.clone().pop().cmp(&b.clone().pop()));
}

fn validate_matching_length(num_placeholders: usize, num_tokens: usize) -> Result<(),TokenizerError> {
	if num_placeholders != num_tokens {
		return Err(TokenizerError::from("mismatched number of unique placeholders and tokens"));
	};

	Ok(())
}

fn validate_placeholders(placeholders: &Vec<String>) -> Result<(),TokenizerError> {
	for (idx, placeholder) in placeholders.iter().enumerate() {
		if placeholder != &format!("?{}",&idx.to_string()) {
			return Err(TokenizerError::from("mismatched placeholder numbers and token vector indexes"));
		};
	}

	Ok(())
}

fn format_token(token: String) -> Result<String,TokenizerError> {
	let number = regex::bytes::Regex::new(r#"^[0-9]+\.?[0-9]*$"#)?;
	if number.is_match(token.as_bytes()) {
		// if string is numeric, use string
		Ok(token)
	} else {
		// if it's not, add quotes
		Ok(format!("{}{}{}","\"",token,"\""))
	}
}

#[derive(Debug)]
pub enum TokenizerError {
	RegexError(regex::Error),
	ConversionError(std::str::Utf8Error),
	Message(&'static str),
}

impl From<regex::Error> for TokenizerError {
	fn from(e: regex::Error) -> Self {
		Self::RegexError(e)
	}
}

impl From<std::str::Utf8Error> for TokenizerError {
	fn from(e: std::str::Utf8Error) -> Self {
		Self::ConversionError(e)
	}
}

impl From<&'static str> for TokenizerError {
	fn from(e: &'static str) -> Self {
		Self::Message(e)
	}
}
