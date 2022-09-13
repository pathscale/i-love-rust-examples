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
pub struct EnumVariant {
    pub name: String,
    pub value: i32,
}
impl EnumVariant {
    pub fn new(name: impl Into<String>, value: i32) -> Self {
        Self {
            name: name.into(),
            value,
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
    Boolean,
    String,
    Bytea,
    UUID,
    Inet,
    Table(String, Vec<Field>),
    DataTable(String, Vec<Field>),
    Vec(Box<Type>),
    Unit,
    Optional(Box<Type>),
    Enum(String, Vec<EnumVariant>),
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
