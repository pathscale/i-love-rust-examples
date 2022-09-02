use crate::endpoint::EndpointSchema;
use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub id: u16,
    pub endpoints: Vec<EndpointSchema>,
}

impl Service {
    pub fn new(name: impl Into<String>, id: u16, endpoints: Vec<EndpointSchema>) -> Self {
        Self {
            name: name.into(),
            id,
            endpoints,
        }
    }
}
