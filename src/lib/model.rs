use serde::*;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Field {
    pub name: String,
    pub ty: Type,
}

impl Field {
    pub fn new(name: impl Into<String>, ty: Type) -> Self {
        Self {
            name: name.into(),
            ty,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Type {
    Second,
    MilliSecond,
    Date,
    Int,
    BigInt,
    Table(String, Vec<Field>),
    DataTable(String, Vec<Field>),
    Vec(Box<Type>),
    Unit,
    Optional(Box<Type>),
}
#[derive(Clone, Debug)]
pub struct ProceduralFunction {
    pub name: String,
    pub parameters: Vec<Field>,
    pub returns: Vec<Field>,
    pub body: String,
}

impl ProceduralFunction {
    pub fn new(
        name: impl Into<String>,
        parameters: Vec<Field>,
        returns: Vec<Field>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            parameters,
            returns,
            body: body.into(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub id: u32,
    pub endpoints: Vec<WsEndpointSchema>,
}

impl Service {
    pub fn new(name: impl Into<String>, id: u32, endpoints: Vec<WsEndpointSchema>) -> Self {
        Self {
            name: name.into(),
            id,
            endpoints,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsEndpointSchema {
    pub name: String,
    pub code: u32,
    pub parameters: Vec<Field>,
    pub returns: Vec<Field>,
    pub json_schema: serde_json::Value,
}

impl WsEndpointSchema {
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
