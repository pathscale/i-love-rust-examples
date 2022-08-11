use crate::sql::ToSql;
use convert_case::{Case, Casing};
use eyre::*;
use itertools::Itertools;
use lib::model::{Field, ProceduralFunction, Type};

pub trait ToRust {
    fn to_rust_ref(&self) -> String;
    fn to_rust_decl(&self) -> String;
}

impl ToRust for Type {
    fn to_rust_ref(&self) -> String {
        match self {
            Type::Second => "u32".to_owned(),
            Type::MilliSecond => "u64".to_owned(),
            Type::Date => "u32".to_owned(), // TODO: resolve date
            Type::Int => "i32".to_owned(),
            Type::BigInt => "i64".to_owned(),
            Type::Table(name, _) => name.clone(),
            Type::DataTable(name, _) => name.clone(),
            Type::Vec(fields) => {
                format!("Vec<{}>", fields.to_rust_ref())
            }
            Type::Unit => "()".to_owned(),
            Type::Optional(t) => {
                format!("Option<{}>", t.to_rust_ref())
            }
        }
    }

    fn to_rust_decl(&self) -> String {
        match self {
            Type::Second => "u32".to_owned(),
            Type::MilliSecond => "u64".to_owned(),
            Type::Date => "u32".to_owned(), // TODO: resolve date
            Type::Int => "i32".to_owned(),
            Type::BigInt => "i64".to_owned(),
            Type::Table(name, fields) => {
                let mut fields = fields
                    .iter()
                    .map(|x| format!("pub {}: {}", x.name, x.ty.to_rust_ref()));
                format!("pub struct {} {{{}}}", name, fields.join(","))
            }
            Type::DataTable(_, _) => {
                todo!()
            }
            Type::Vec(fields) => {
                format!("Vec<{}>", fields.to_rust_ref())
            }
            Type::Unit => "()".to_owned(),
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
            let rows = self
                .client
                .query(
                    "SELECT * FROM api.fun_user_foo( a_arg1 => $1::String );",
                    &[&req.arg1],
                )
                .await?;
            let mut resp = FunUserFooResp {
                rows: Vec::with_capacity(rows.len()),
            };
            for row in rows {
                let r = FunUserFooRespRow {
                    arg1: row.try_get(0)?,
                };
                resp.rows.push(r);
            }
            Ok(resp)
        }
    }
}

pub fn get_parameter_type(this: &ProceduralFunction) -> Type {
    Type::Table(
        format!("{}Req", this.name.to_case(Case::Pascal)),
        this.parameters.clone(),
    )
}
pub fn get_return_row_type(this: &ProceduralFunction) -> Type {
    Type::Table(
        format!("{}RespRow", this.name.to_case(Case::Pascal)),
        this.returns.clone(),
    )
}
pub fn get_return_type(this: &ProceduralFunction) -> Type {
    Type::Table(
        format!("{}Resp", this.name.to_case(Case::Pascal)),
        vec![Field::new(
            "rows",
            Type::Vec(Box::new(get_return_row_type(this))),
        )],
    )
}
pub fn to_rust_type_decl(this: &ProceduralFunction) -> String {
    [
        get_parameter_type(this),
        get_return_row_type(this),
        get_return_type(this),
    ]
    .map(|x| x.to_rust_decl())
    .join("\n")
}
pub fn to_rust_decl(this: &ProceduralFunction) -> String {
    let mut arguments = this
        .parameters
        .iter()
        .enumerate()
        .map(|(i, x)| format!("{} => {}::{}", x.name, i, x.ty.to_sql()));
    let sql = format!("SELECT * FROM api.{}({});", this.name, arguments.join(","));
    let pg_params = this
        .parameters
        .iter()
        .map(|x| format!("&req.{}", x.name))
        .join(",");
    let row_getter = this
        .returns
        .iter()
        .enumerate()
        .map(|(i, x)| format!("{}: row.try_get({})?", x.name, i + 1))
        .join(",");
    format!(
        "pub async fn {name_raw}(&self, req: {name}Req) -> Result<{name}Resp> {{
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
        name_raw = this.name,
        name = this.name.to_case(Case::Pascal),
        sql = sql,
        pg_params = pg_params,
        row_getter = row_getter
    )
}
