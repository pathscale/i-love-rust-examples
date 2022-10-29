use itertools::Itertools;
use thiserror::Error;

pub fn tokenize_statements(
    statements: &str,
    tokens: Vec<String>,
) -> Result<String, TokenizerError> {
    let (num_placeholders, mut placeholders) = unique_placeholders(statements)?;

    order_placeholders(&mut placeholders)?;
    validate_matching_length(num_placeholders, tokens.len())?;
    if tokens.len() == 0 {
        return Ok(statements.to_owned());
    };
    validate_placeholders(&placeholders)?;

    let mut tokenized_statements: String = statements.to_owned();
    for (idx, placeholder) in placeholders.iter().enumerate().rev() {
        tokenized_statements =
            tokenized_statements.replace(placeholder, &format_token(tokens[idx].to_owned())?);
    }

    Ok(tokenized_statements)
}

fn unique_placeholders(statements: &str) -> Result<(usize, Vec<String>), TokenizerError> {
    let token_placeholders = regex::bytes::Regex::new(r#"(?:\?[0-9]+)"#)?;
    let captured_matches = token_placeholders.captures_iter(statements.as_bytes());

    let mut matches: Vec<&[u8]> = Vec::new();
    let mut placeholders: Vec<String> = Vec::new();
    for capture in captured_matches {
        let match_bytes = capture
            .get(0)
            .ok_or(TokenizerError::RegexCaptureError)?
            .as_bytes();
        matches.push(match_bytes);

        let placeholder = std::str::from_utf8(match_bytes)?;
        placeholders.push(placeholder.to_owned());
    }

    Ok((
        matches.iter().unique().count(),
        placeholders.into_iter().unique().collect(),
    ))
}

fn order_placeholders(placeholders: &mut Vec<String>) -> Result<(), TokenizerError> {
    let mut failed: Option<std::num::ParseIntError> = None;
    placeholders.sort_by(|a, b| {
        let mut a_clone = a.clone();
        let mut b_clone = b.clone();
        a_clone.remove(0);
        b_clone.remove(0);
        let a_int = a_clone
            .parse::<u32>()
            .map_err(|e| failed = Some(e))
            .unwrap_or_default();
        let b_int = b_clone
            .parse::<u32>()
            .map_err(|e| failed = Some(e))
            .unwrap_or_default();
        a_int.cmp(&b_int)
    });
    match failed {
        Some(e) => Err(e.into()),
        None => Ok(()),
    }
}

fn validate_matching_length(
    num_placeholders: usize,
    num_tokens: usize,
) -> Result<(), TokenizerError> {
    if num_placeholders != num_tokens {
        return Err(TokenizerError::DifferentNumberPlaceholdersAndTokensError);
    };

    Ok(())
}

fn validate_placeholders(placeholders: &Vec<String>) -> Result<(), TokenizerError> {
    for (idx, placeholder) in placeholders.iter().enumerate() {
        if placeholder != &format!("?{}", &idx.to_string()) {
            return Err(TokenizerError::MismatchedPlaceholdersAndTokensError);
        };
    }

    Ok(())
}

fn format_token(token: String) -> Result<String, TokenizerError> {
    let forced_string = regex::bytes::Regex::new(r#"^--force-string"#)?;
    let number = regex::bytes::Regex::new(r#"^[0-9]+\.?[0-9]*$"#)?;
    let boolean = regex::bytes::Regex::new(r#"^true|false$"#)?;
    let token_bytes = token.as_bytes();
    if forced_string.is_match(token_bytes) {
        // if it has a forced flag, add quotes to whatever it is
        Ok(format!(
            "{}{}{}",
            "\"",
            token.replace("--force-string", ""),
            "\""
        ))
    } else if number.is_match(token_bytes) {
        // if string is numeric, use raw
        Ok(token)
    } else if boolean.is_match(token_bytes) {
        // if string is a boolean, use raw
        Ok(token)
    } else {
        // if it's not, add quotes
        Ok(format!("{}{}{}", "\"", token, "\""))
    }
}

#[derive(Debug, Error)]
pub enum TokenizerError {
    #[error("regex instantiation failed")]
    RegexInstantiationError(#[from] regex::Error),
    #[error("regex capture failed")]
    RegexCaptureError,
    #[error("different number of placeholders and tokens")]
    DifferentNumberPlaceholdersAndTokensError,
    #[error("placeholder numbers don't match token indexes")]
    MismatchedPlaceholdersAndTokensError,
    #[error("failed to convert regex match bytes to string")]
    BytesToStringConversionError(#[from] std::str::Utf8Error),
    #[error("failed to parse placeholder to int")]
    PlaceholderParseError(#[from] std::num::ParseIntError),
}
