#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

pub struct Service {
    pub name: String,
    pub id: u32,
}

impl Service {
    pub fn new(name: impl Into<String>, id: u32) -> Self {
        Self {
            name: name.into(),
            id,
        }
    }
}
