pub mod rust;
pub mod sql;

use crate::rust::{to_rust_decl, to_rust_type_decl};
use crate::sql::ToSql;
use eyre::*;
use lib::model::*;
use std::fs::File;
use std::io::Write;
use std::process::Command;
pub const SYMBOL: &str = "a_";

include!("../service/services.rs");

fn gen_db_rs() -> Result<()> {
    let funcs = get_proc_functions();

    let db_filename = "src/gen/database.rs";
    let mut db = File::create(db_filename)?;

    write!(
        &mut db,
        "{}",
        r#"
use tokio_postgres::*;
use eyre::*;
pub struct DatabaseClient {
    client: Client
}
impl DatabaseClient {
    pub fn new(client: Client) -> Self {
        Self {
            client
        }
    }
}
    "#
    )?;
    for func in funcs {
        write!(
            &mut db,
            "
{}
impl DatabaseClient {{ 
    {}
}}",
            to_rust_type_decl(&func),
            to_rust_decl(&func)
        )?;
    }
    db.flush()?;
    drop(db);
    let exit = Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg(db_filename)
        .spawn()?
        .wait()?;
    if !exit.success() {
        bail!("failed to rustfmt {} {:?}", db_filename, exit);
    }
    Ok(())
}

fn gen_db_sql() -> Result<()> {
    let funcs = get_proc_functions();

    let db_filename = "db/api.sql";
    let mut db = File::create(db_filename)?;
    for func in funcs {
        writeln!(&mut db, "{}", func.to_sql())?;
    }
    db.flush()?;
    drop(db);

    Ok(())
}
fn gen_docs() -> Result<()> {
    let services = get_services();
    let docs_filename = "docs/services.json";
    let mut docs_file = File::create(docs_filename)?;
    serde_json::to_writer_pretty(&mut docs_file, &services)?;
    Ok(())
}
fn main() -> Result<()> {
    gen_docs()?;
    gen_db_sql()?;
    gen_db_rs()?;
    Ok(())
}
