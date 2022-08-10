use itertools::{Itertools};
use crate::model::{ProceduralFunction, Type};

pub trait ToSql {
    fn to_sql(&self) -> String;
}

impl ToSql for Type {
    fn to_sql(&self) -> String {
        match self {
            Type::Second => { "oid".to_owned() }
            Type::MilliSecond => { "int".to_owned() }
            Type::Date => { "int".to_owned() } // TODO: fix things
            Type::Int => { "int".to_owned() }
            Type::BigInt => { "bigint".to_owned() }
            Type::Table(_, fields) => {
                let mut fields = fields.iter().map(|x| format!("{} {}", x.0, x.1.to_sql()));
                format!("table ({})", fields.join(","))
            }
            Type::DataTable(_, _) => { todo!() }
            Type::Vec(fields) => {
                format!("{}[]", fields.to_sql())
            }
            Type::Unit => {
                "void".to_owned()
            }
            Type::Optional(t) => {
                format!("Option<{}>", t.to_sql())
            }
        }
    }
}


impl ToSql for ProceduralFunction {
    fn to_sql(&self) -> String {
        let params = self.parameters.iter().map(|x| match &x.1 {
            Type::Optional(y) => { format!("a_{} {}=NULL", x.0, y.to_sql()) }
            y => format!("a_{} {}", x.0, y.to_sql())
        }).join(",");
        format!("CREATE OR REPLACE FUNCTION api.{name}({params})
            RETURNS {returns}
            LANGUAGE plpgsql
            AS $$
                {body}
            $$;
        ",
                name = self.name,
                params = params,
                returns = Type::Table("".to_string(), self.returns.clone()).to_sql(),
                body = self.body
        )
    }
}