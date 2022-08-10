pub type Field = (String, Type);
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
    Optional(Box<Type>)
}


pub struct ProceduralFunction {
    pub name: String,
    pub parameters: Vec<Field>,
    pub returns: Vec<Field>,
    pub body: String,
}


