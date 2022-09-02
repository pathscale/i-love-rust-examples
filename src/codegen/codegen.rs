pub mod rust;
pub mod service;
pub mod sql;
use crate::rust::{to_rust_decl, to_rust_type_decl, ToRust};
use crate::service::get_systemd_service;
use crate::sql::ToSql;
use convert_case::{Case, Casing};
use eyre::*;
use itertools::Itertools;
use model::endpoint::*;
use model::service::*;
use model::types::*;
use serde::*;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::process::Command;

pub const SYMBOL: &str = "a_";

include!("../service/services.rs");
include!("../service/enums.rs");

pub fn gen_model_rs(dir: &str) -> Result<()> {
    let db_filename = format!("{}/model.rs", dir);
    let mut f = File::create(&db_filename)?;

    write!(
        &mut f,
        "{}",
        r#"
use tokio_postgres::types::*;
use serde::*;

    "#
    )?;
    for e in get_enums() {
        write!(&mut f, "{}", e.to_rust_decl())?;
    }
    f.flush()?;
    write!(
        &mut f,
        "{}",
        Type::Enum(
            "service".to_owned(),
            get_services()
                .into_iter()
                .map(|x| EnumVariant::new(x.name, x.id as _))
                .collect()
        )
        .to_rust_decl()
    )?;

    drop(f);
    rustfmt(&db_filename)?;

    Ok(())
}
pub fn gen_model_sql(root: &str) -> Result<()> {
    let db_filename = format!("{}/db/model.sql", root);
    let mut f = File::create(db_filename)?;

    write!(&mut f, "{}", r#"CREATE SCHEMA IF NOT EXISTS tbl;"#)?;
    let mut enums = get_enums();

    enums.push(Type::Enum(
        "service".to_owned(),
        get_services()
            .into_iter()
            .map(|x| EnumVariant::new(x.name, x.id as _))
            .collect(),
    ));

    for e in enums {
        match e {
            Type::Enum(name, field) => {
                writeln!(
                    &mut f,
                    "CREATE TYPE tbl.enum_{} AS ENUM ({});",
                    name,
                    field
                        .into_iter()
                        .map(|x| format!("'{}'", x.name))
                        .join(", ")
                )?;
            }
            _ => unreachable!(),
        }
    }
    f.flush()?;
    drop(f);
    Ok(())
}
pub fn rustfmt(f: &str) -> Result<()> {
    let exit = Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg(f)
        .spawn()?
        .wait()?;
    if !exit.success() {
        bail!("failed to rustfmt {:?}", exit);
    }
    Ok(())
}
pub fn gen_db_rs(dir: &str) -> Result<()> {
    let funcs = get_proc_functions();

    let db_filename = format!("{}/database.rs", dir);
    let mut db = File::create(&db_filename)?;

    write!(
        &mut db,
        "{}",
        r#"
use eyre::*;
use lib::database::*;
use crate::model::*;
#[derive(Clone)]
pub struct DbClient {
    client: SimpleDbClient
}
impl DbClient {
    pub fn new(client: SimpleDbClient) -> Self {
        Self {
            client
        }
    }
}
impl From<SimpleDbClient> for DbClient {
    fn from(client: SimpleDbClient) -> Self {
        Self::new(client)
    }
}
    "#
    )?;
    for func in funcs {
        write!(
            &mut db,
            "
{}
impl DbClient {{ 
    #[allow(unused_variables)]
    {}
}}",
            to_rust_type_decl(&func),
            to_rust_decl(&func)
        )?;
    }
    db.flush()?;
    drop(db);
    rustfmt(&db_filename)?;
    Ok(())
}

pub fn gen_db_sql(root: &str) -> Result<()> {
    let funcs = get_proc_functions();

    let db_filename = format!("{}/db/api.sql", root);
    let mut f = File::create(&db_filename)?;
    writeln!(&mut f, "{}", r#"CREATE SCHEMA IF NOT EXISTS api;"#)?;
    for func in funcs {
        writeln!(&mut f, "{}", func.to_sql())?;
    }
    for srv in get_services() {
        writeln!(
            &mut f,
            "{}",
            ProceduralFunction::new(
                format!("{}_SERVICE", srv.name.to_case(Case::ScreamingSnake)),
                vec![],
                vec![Field::new("code", Type::Int)],
                format!("BEGIN RETURN QUERY SELECT {}; END", srv.id),
            )
            .to_sql()
        )?;
    }
    f.flush()?;
    drop(f);

    Ok(())
}
pub fn gen_docs(root: &str) -> Result<()> {
    let services = get_services();
    let docs_filename = format!("{}/docs/services.json", root);
    let mut docs_file = File::create(docs_filename)?;
    serde_json::to_writer_pretty(&mut docs_file, &services)?;
    Ok(())
}

pub fn gen_systemd_services(root: &str, app_name: &str, user: &str, port: u16) -> Result<()> {
    create_dir_all(format!("{}/etc/systemd", root))?;
    let services = get_services();
    for srv in services {
        let service_filename = format!("{}/etc/systemd/{}.service", root, srv.name);
        let mut service_file = File::create(&service_filename)?;
        let v = get_systemd_service(app_name, &srv.name, user, port + srv.id);
        write!(&mut service_file, "{}", v)?;
    }
    Ok(())
}
pub fn main() -> Result<()> {
    let mut root = env::current_dir()?;
    loop {
        if root.join(".cargo").exists() {
            break;
        }
        root = root.parent().unwrap().to_owned();
    }
    let root = root.to_str().unwrap();
    // let dir = env::var_os("OUT_DIR")
    //     .map(|x| x.to_str().unwrap().to_owned())
    //     .unwrap_or(format!("{}/target/gen", root));
    let dir = format!("{}/src/gen", root);
    create_dir_all(&dir)?;
    gen_docs(root)?;
    gen_model_rs(&dir)?;
    gen_model_sql(root)?;
    gen_db_sql(root)?;
    gen_db_rs(&dir)?;
    gen_systemd_services(root, "coldvaults", "jack", 7500)?;
    Ok(())
}
