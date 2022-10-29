use super::{Row, Value};

pub trait ParsableRow {
    fn maybe_first_value(&mut self) -> Option<Value>;
    fn try_first_value(&mut self) -> Result<Value, ParseRowError>;
}

impl ParsableRow for Row {
    fn maybe_first_value(&mut self) -> Option<Value> {
        if self.is_empty() {
            None
        } else {
            Some(self[0].clone())
        }
    }

    fn try_first_value(&mut self) -> Result<Value, ParseRowError> {
        if self.is_empty() {
            Err(ParseRowError::EmptyRow)
        } else {
            Ok(self[0].clone())
        }
    }
}

#[derive(Debug)]
pub enum ParseRowError {
    EmptyRow,
}

impl std::fmt::Display for ParseRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyRow => write!(f, "{:?}", "row is empty"),
        }
    }
}

impl std::error::Error for ParseRowError {}
