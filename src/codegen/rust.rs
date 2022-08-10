use itertools::{Itertools};
use crate::model::{ProceduralFunction, Type};
use eyre::*;
use crate::sql::ToSql;

pub trait ToRust {
    fn to_rust_ref(&self) -> String;
    fn to_rust_decl(&self) -> String;
}

impl ToRust for Type {
    fn to_rust_ref(&self) -> String {
        match self {
            Type::Second => { "u32".to_owned() }
            Type::MilliSecond => { "u64".to_owned() }
            Type::Date => { "u32".to_owned() } // TODO: resolve date
            Type::Int => { "i32".to_owned() }
            Type::BigInt => { "i64".to_owned() }
            Type::Table(name, _) => {
                name.clone()
            }
            Type::DataTable(name, _) => { name.clone() }
            Type::Vec(fields) => {
                format!("Vec<{}>", fields.to_rust_ref())
            }
            Type::Unit => {
                "()".to_owned()
            }
            Type::Optional(t) => {
                format!("Option<{}>", t.to_rust_ref())
            }
        }
    }

    fn to_rust_decl(&self) -> String {
        match self {
            Type::Second => { "u32".to_owned() }
            Type::MilliSecond => { "u64".to_owned() }
            Type::Date => { "u32".to_owned() } // TODO: resolve date
            Type::Int => { "i32".to_owned() }
            Type::BigInt => { "i64".to_owned() }
            Type::Table(name, fields) => {
                let mut fields = fields.iter().map(|x| format!("pub {}: {}", x.0, x.1.to_rust_ref()));
                format!("pub struct {} {{{}}}", name, fields.join(","))
            }
            Type::DataTable(_, _) => { todo!() }
            Type::Vec(fields) => {
                format!("Vec<{}>", fields.to_rust_ref())
            }
            Type::Unit => {
                "()".to_owned()
            }
            Type::Optional(t) => {
                format!("Option<{}>", t.to_rust_ref())
            }
        }
    }
}

#[allow(unused)]
mod example {
    use super::*;
    struct DatabaseMock {
        client: tokio_postgres::Client,
    }

    struct FunUserFooReq {
        arg1: String,
    }

    struct FunUserFooRespRow {
        arg1: String,
    }

    struct FunUserFooResp {
        rows: Vec<FunUserFooRespRow>,
    }

    impl DatabaseMock {
        pub async fn fun_user_foo(&self, req: FunUserFooReq) -> Result<FunUserFooResp> {
            let rows = self.client.query("SELECT * FROM api.fun_user_foo( a_arg1 => $1::String );", &[&req.arg1]).await?;
            let mut resp = FunUserFooResp {
                rows: Vec::with_capacity(rows.len())
            };
            for row in rows {
                let r = FunUserFooRespRow {
                    arg1: row.try_get(0)?
                };
                resp.rows.push(r);
            }
            Ok(resp)
        }
    }
}

impl ProceduralFunction {
    pub fn get_parameter_type(&self) -> Type {
        Type::Table(format!("{}Req", self.name), self.parameters.clone())
    }
    pub fn get_return_row_type(&self) -> Type {
        Type::Table(format!("{}RespRow", self.name), self.returns.clone())
    }
    pub fn get_return_type(&self) -> Type {
        Type::Table(format!("{}Resp", self.name), vec![("rows".to_owned(), Type::Vec(Box::new(self.get_return_row_type())))])
    }
}

impl ToRust for ProceduralFunction {
    fn to_rust_ref(&self) -> String {
        unreachable!()
    }

    fn to_rust_decl(&self) -> String {
        let mut arguments = self.parameters.iter().enumerate().map(|(i, x)| format!("{} => {}::{}", x.0, i, x.1.to_sql()));
        let sql = format!("SELECT * FROM api.{}({});", self.name, arguments.join(","));
        let pg_params = self.parameters.iter().map(|x| format!("&req.{}", x.0)).join(",");
        let row_getter = self.returns.iter().enumerate().map(|(i, _)| format!("arg1: row.try_get({})?", i)).join(",");
        format!("pub async fn {name}(&self, req: {name}Req) -> Result<{name}Resp> {{
          let rows = self.client.query(\"{sql}\", &[{pg_params}]).await?;
          let mut resp = {name}Resp {{
              rows: Vec::with_capacity(rows.len())
          }};
          for row in rows {{
            let r = {name}RespRow {{
              {row_getter}
            }};
            resp.rows.push(r);
          }}
          Ok(resp)
        }}",
                name = self.name,
                sql = sql,
                pg_params = pg_params,
                row_getter = row_getter
        )
    }
}