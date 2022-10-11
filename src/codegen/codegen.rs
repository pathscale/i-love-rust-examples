pub mod rust;
pub mod service;
pub mod sql;

use crate::rust::{to_rust_decl, to_rust_type_decl, ToRust};
use crate::service::get_systemd_service;
use crate::sql::ToSql;
use convert_case::{Case, Casing};
use eyre::*;
use itertools::Itertools;
use model::service::Service;
use model::types::*;
use serde::*;
use std::collections::HashMap;
use std::env;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::process::Command;

pub const SYMBOL: &str = "a_";
#[path = "../service/services.rs"]
mod services;

#[path = "../service/enums.rs"]
mod enums;

pub fn collect_rust_recursive_types(t: Type) -> Vec<Type> {
    match t {
        Type::Object { ref fields, .. } => {
            let mut v = vec![t.clone()];
            for x in fields {
                v.extend(collect_rust_recursive_types(x.ty.clone()));
            }
            v
        }
        Type::DataTable { name, fields } => {
            collect_rust_recursive_types(Type::object(name, fields))
        }
        Type::Vec(x) => collect_rust_recursive_types(*x),
        _ => vec![],
    }
}

pub fn check_endpoint_codes() -> Result<()> {
    let mut codes = HashMap::new();
    for s in services::get_services() {
        for e in s.endpoints {
            let code = e.code;
            if codes.contains_key(&code) {
                bail!("duplicate service code: {} {} {}", s.name, e.name, e.code);
            }
            codes.insert(code, e.code);
        }
    }
    Ok(())
}

pub fn gen_model_rs(dir: &str) -> Result<()> {
    let db_filename = format!("{}/model.rs", dir);
    let mut f = File::create(&db_filename)?;

    write!(
        &mut f,
        "{}",
        r#"
use tokio_postgres::types::*;
use serde::*;
use num_derive::FromPrimitive;
use strum_macros::EnumString;

    "#
    )?;
    for e in enums::get_enums() {
        write!(&mut f, "{}", e.to_rust_decl())?;
    }

    for s in services::get_services() {
        for e in s.endpoints {
            let req = Type::object(format!("{}Request", e.name), e.parameters);
            let resp = Type::object(format!("{}Response", e.name), e.returns);
            let ss = vec![
                collect_rust_recursive_types(req),
                collect_rust_recursive_types(resp),
            ]
            .concat();
            for s in ss {
                write!(
                    &mut f,
                    r#"#[derive(Serialize, Deserialize, Debug)]
                    #[serde(rename_all = "camelCase")]
                    {}"#,
                    s.to_rust_decl()
                )?;
            }
        }
    }
    f.flush()?;
    drop(f);
    rustfmt(&db_filename)?;

    Ok(())
}
pub fn gen_model_sql(root: &str) -> Result<()> {
    let db_filename = format!("{}/db/model.sql", root);
    let mut f = File::create(db_filename)?;

    for e in enums::get_enums() {
        match e {
            Type::Enum { name, variants } => {
                writeln!(
                    &mut f,
                    "CREATE TYPE enum_{} AS ENUM ({});",
                    name,
                    variants
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
    let funcs = services::get_repo_functions();

    let db_filename = format!("{}/database.rs", dir);
    let mut db = File::create(&db_filename)?;

    write!(
        &mut db,
        "{}",
        r#"
    use eyre::*;
    use lib::database::*;
    use crate::model::*;
		"#
    )?;

    for func in funcs {
        write!(&mut db, "{}", to_rust_type_decl(&func),)?;
    }
    db.flush()?;
    drop(db);
    rustfmt(&db_filename)?;
    Ok(())
}

pub fn gen_db_sql(root: &str) -> Result<()> {
    let funcs = services::get_proc_functions();

    let db_filename = format!("{}/db/api.sql", root);
    let mut f = File::create(&db_filename)?;
    writeln!(&mut f, "{}", r#"CREATE SCHEMA IF NOT EXISTS api;"#)?;
    for func in funcs {
        writeln!(&mut f, "{}", func.to_sql())?;
    }
    for srv in services::get_services() {
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
#[derive(Debug, Serialize, Deserialize)]
struct Docs {
    services: Vec<Service>,
    enums: Vec<Type>,
}
pub fn gen_docs(root: &str) -> Result<()> {
    let docs = Docs {
        services: services::get_services(),
        enums: enums::get_enums(),
    };
    let docs_filename = format!("{}/docs/services.json", root);
    let mut docs_file = File::create(docs_filename)?;
    serde_json::to_writer_pretty(&mut docs_file, &docs)?;
    Ok(())
}

pub fn gen_systemd_services(
    root: &str,
    app_name: &str,
    user: &str,
    host: HashMap<String, String>,
) -> Result<()> {
    create_dir_all(format!("{}/etc/systemd", root))?;
    let services = services::get_services();
    for srv in services {
        let service_filename = format!("{}/etc/systemd/{}_{}.service", root, app_name, srv.name);
        let mut service_file = File::create(&service_filename)?;
        let v = get_systemd_service(
            app_name,
            &srv.name,
            user,
            &host
                .get(&srv.name)
                .ok_or_else(|| eyre!("Could not find key {}", srv.name))?,
            443,
        );
        write!(&mut service_file, "{}", v)?;
    }
    Ok(())
}
pub fn main() -> Result<()> {
    check_endpoint_codes()?;
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
    gen_systemd_services(
        root,
        "iloverust",
        "ilr",
        HashMap::from([
            ("auth".to_owned(), "auth.iloverust".to_owned()),
            ("user".to_owned(), "user.iloverust".to_owned()),
            ("admin".to_owned(), "admin.iloverust".to_owned()),
        ]),
    )?;
    Ok(())
}
