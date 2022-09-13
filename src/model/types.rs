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
    pub value: i64,
}
impl EnumVariant {
    pub fn new(name: impl Into<String>, value: i64) -> Self {
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
    Numeric,
    Boolean,
    String,
    Bytea,
    UUID,
    Inet,
    Object {
        name: String,
        fields: Vec<Field>,
    },
    DataTable {
        name: String,
        fields: Vec<Field>,
    },
    Vec(Box<Type>),
    Unit,
    Optional(Box<Type>),
    Enum {
        name: String,
        variants: Vec<EnumVariant>,
    },
}
impl Type {
    pub fn object(name: impl Into<String>, fields: Vec<Field>) -> Self {
        Self::Object {
            name: name.into(),
            fields,
        }
    }
    pub fn data_table(name: impl Into<String>, fields: Vec<Field>) -> Self {
        Self::DataTable {
            name: name.into(),
            fields,
        }
    }
    pub fn vec(ty: Type) -> Self {
        Self::Vec(Box::new(ty))
    }
    pub fn optional(ty: Type) -> Self {
        Self::Optional(Box::new(ty))
    }
    pub fn enum_ref(name: impl Into<String>) -> Self {
        Self::Enum {
            name: name.into(),
            variants: vec![],
        }
    }
    pub fn enum_(name: impl Into<String>, fields: Vec<EnumVariant>) -> Self {
        Self::Enum {
            name: name.into(),
            variants: fields,
        }
    }
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
