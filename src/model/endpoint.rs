use crate::types::*;
use serde::*;
#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointSchema {
    pub name: String,
    pub code: u32,
    pub parameters: Vec<Field>,
    pub returns: Vec<Field>,
    pub json_schema: serde_json::Value,
}

impl EndpointSchema {
    pub fn new(
        name: impl Into<String>,
        code: u32,
        parameters: Vec<Field>,
        returns: Vec<Field>,
    ) -> Self {
        Self {
            name: name.into(),
            code,
            parameters,
            returns,
            json_schema: Default::default(),
        }
    }
}
