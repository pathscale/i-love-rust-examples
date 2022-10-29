use thiserror::Error;

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

#[derive(Debug, Error)]
pub enum ParseRowError {
    #[error("empty row")]
    EmptyRow,
}
