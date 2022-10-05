use crate::log::LogLevel;
use clap::Parser;
use eyre::*;
use serde::*;
use std::env::current_dir;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CliArgument {
    /// The path to config file
    #[clap(
        short,
        long,
        value_parser,
        value_name = "FILE",
        default_value = "etc/config.json",
        env = "CONFIG"
    )]
    config: PathBuf,
    #[clap(long, env = "LOG_LEVEL")]
    log_level: Option<LogLevel>,
    #[clap(long, env = "HOST")]
    /// The host to listen on
    host: Option<String>,
    #[clap(long, env = "PORT")]
    /// The port to listen on
    port: Option<u16>,
    #[clap(long, env = "PUB_CERT")]
    pub_cert: Option<String>,
    #[clap(long, env = "PRIV_CERT")]
    priv_cert: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub db: DbConfig,
    pub app: AppConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub name: String,
    pub log_level: LogLevel,
    pub host: String,
    pub port: u16,
    pub pub_cert: String,
    pub priv_cert: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppFileConfig {
    pub log_level: LogLevel,
    pub host: String,
    pub port: u16,
    pub pub_cert: String,
    pub priv_cert: String,
}

pub fn load_config(service_name: String) -> Result<Config> {
    let args: CliArgument = CliArgument::parse();

    println!("Working directory {}", current_dir()?.display());
    println!("Loading config from {}", args.config.display());

    let config = std::fs::read_to_string(&args.config)?;
    let root: serde_json::Value = serde_json::from_str(&config)?;

    let db: DbConfig = serde_json::from_value(root.get("db").unwrap().to_owned())?;
    let app_file: AppFileConfig = serde_json::from_value(
        root.get("app")
            .unwrap()
            .get(&service_name)
            .unwrap()
            .to_owned(),
    )?;

    let mut app: AppConfig = AppConfig::default();
    app.log_level = if args.log_level.is_some() {
        args.log_level.unwrap()
    } else {
        app_file.log_level
    };
    app.port = if args.port.is_some() {
        args.port.unwrap()
    } else {
        app_file.port
    };
    app.host = if args.host.is_some() {
        args.host.unwrap()
    } else {
        app_file.host
    };
    app.pub_cert = if args.pub_cert.is_some() {
        args.pub_cert.unwrap()
    } else {
        app_file.pub_cert
    };
    app.priv_cert = if args.priv_cert.is_some() {
        args.priv_cert.unwrap()
    } else {
        app_file.priv_cert
    };
    app.name = service_name;
    println!("App config {:#?}", app);

    Ok(Config { db: db, app: app })
}
