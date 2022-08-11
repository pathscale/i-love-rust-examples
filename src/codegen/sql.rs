use crate::SYMBOL;
use itertools::Itertools;
use lib::model::{ProceduralFunction, Type};

pub trait ToSql {
    fn to_sql(&self) -> String;
}

impl ToSql for Type {
    fn to_sql(&self) -> String {
        match self {
            Type::Second => "oid".to_owned(),
            Type::MilliSecond => "int".to_owned(),
            Type::Date => "int".to_owned(), // TODO: fix things
            Type::Int => "int".to_owned(),
            Type::BigInt => "bigint".to_owned(),
            Type::Table(_, fields) => {
                let mut fields = fields
                    .iter()
                    .map(|x| format!("{} {}", x.name, x.ty.to_sql()));
                format!("table (\n{}\n)", fields.join(",\n"))
            }
            Type::DataTable(_, _) => {
                todo!()
            }
            Type::Vec(fields) => {
                format!("{}[]", fields.to_sql())
            }
            Type::Unit => "void".to_owned(),
            Type::Optional(t) => {
                format!("Option<{}>", t.to_sql())
            }
        }
    }
}
impl ToSql for ProceduralFunction {
    fn to_sql(&self) -> String {
        let params = self
            .parameters
            .iter()
            .map(|x| match &x.ty {
                Type::Optional(y) => {
                    format!("{}{} {}=NULL", SYMBOL, x.name, y.to_sql())
                }
                y => format!("{}{} {}", SYMBOL, x.name, y.to_sql()),
            })
            .join(", ");
        format!(
            "
CREATE OR REPLACE FUNCTION api.{name}({params})
RETURNS {returns}
LANGUAGE plpgsql
AS $$
    {body}
$$;
        ",
            name = self.name,
            params = params,
            returns = Type::Table("".to_string(), self.returns.clone()).to_sql(),
            body = self.body.replace("$", SYMBOL)
        )
    }
}
