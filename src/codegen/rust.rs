use crate::sql::ToSql;
use crate::SYMBOL;
use convert_case::{Case, Casing};
use itertools::Itertools;
use model::types::*;

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
            Type::Boolean => "bool".to_owned(),
            Type::String => "String".to_owned(),
            Type::Bytea => "Vec<u8>".to_owned(),
            Type::UUID => "uuid::Uuid".to_owned(),
            Type::Inet => "std::net::IpAddr".to_owned(),
            Type::Enum(name, _) => format!("Enum{}", name.to_case(Case::Pascal),),
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
            Type::Boolean => "bool".to_owned(),
            Type::String => "String".to_owned(),
            Type::Bytea => "Vec<u8>".to_owned(),
            Type::UUID => "uuid::Uuid".to_owned(),
            Type::Inet => "std::net::IpAddr".to_owned(),
            Type::Enum(name, fields) => {
                let mut fields = fields.iter().map(|x| {
                    format!(
                        r#"#[postgres(name = "{}")]{} = {}"#,
                        x.name,
                        x.name.to_case(Case::Pascal),
                        x.value
                    )
                });
                format!(
                    r#"#[derive(Debug, Clone, Copy, ToSql, FromSql, Serialize, Deserialize, FromPrimitive, PartialEq, EnumString)] #[postgres(name = "enum_{}")]pub enum Enum{} {{{}}}"#,
                    name,
                    name.to_case(Case::Pascal),
                    fields.join(",")
                )
            }
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
        .map(|(i, x)| format!("{}{} => ${}::{}", SYMBOL, x.name, i + 1, x.ty.to_sql()));
    let sql = format!("SELECT * FROM api.{}({});", this.name, arguments.join(", "));
    let pg_params = this
        .parameters
        .iter()
        .map(|x| format!("&req.{}", x.name))
        .join(", ");
    let row_getter = this
        .returns
        .iter()
        .enumerate()
        .map(|(i, x)| format!("{}: row.try_get({})?", x.name, i))
        .join(",\n");
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
