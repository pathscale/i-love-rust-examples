use super::Row;

#[derive(Debug, Default)]
pub struct SelectPayload {
    pub labels: Vec<String>,
    pub rows: Vec<Row>,
}

impl SelectPayload {
    pub fn maybe_first_row(&mut self) -> Option<Row> {
        if self.rows.is_empty() {
            None
        } else {
            Some(self.rows[0].clone())
        }
    }

    pub fn try_first_row(&mut self) -> Result<Row, ParseSelectPayloadError> {
        if self.rows.is_empty() {
            Err(ParseSelectPayloadError::EmptyRows)
        } else {
            Ok(self.rows[0].clone())
        }
    }
}

#[derive(Debug)]
pub enum ParseSelectPayloadError {
    EmptyRows,
}

impl std::fmt::Display for ParseSelectPayloadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyRows => write!(f, "{:?}", "no rows in select payload"),
        }
    }
}

impl std::error::Error for ParseSelectPayloadError {}
