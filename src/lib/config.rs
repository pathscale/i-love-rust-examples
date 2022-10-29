use crate::log::LogLevel;
use clap::Parser;
use eyre::*;
use serde::*;
use std::collections::HashMap;
use std::env::current_dir;
use std::fmt::Debug;
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
    #[clap(long, env = "DEBUG")]
    debug: bool,
    #[clap(long, env = "PUB_CERTS", value_delimiter = ',')]
    pub_certs: Option<Vec<String>>,
    #[clap(long, env = "PRIV_CERT")]
    priv_cert: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub port: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub db: DbConfig,
    #[serde(skip)]
    pub app: AppConfig,
    #[serde(flatten)]
    pub apps: HashMap<String, AppConfig>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub log_level: LogLevel,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub pub_certs: Option<Vec<String>>,
    #[serde(default)]
    pub priv_cert: Option<String>,
    #[serde(default)]
    pub debug: bool,
    #[serde(skip)]
    pub header_only: bool,
}
pub fn load_config(service_name: String) -> Result<Config> {
    let args: CliArgument = CliArgument::parse();

    println!("Working directory {}", current_dir()?.display());
    println!("Loading config from {}", args.config.display());
    let config = std::fs::read_to_string(&args.config)?;

    let mut config: Config = serde_json::from_str(&config)?;

    println!("CONFIG: {:?}", config);

    if let Some(a) = config.apps.get(&service_name) {
        config.app = serde_json::from_value(serde_json::to_value(a)?)?;
    }

    // merge configs
    if let Some(log_level) = args.log_level {
        config.app.log_level = log_level;
    }
    if let Some(host) = args.host {
        config.app.host = host;
    }
    if let Some(port) = args.port {
        config.app.port = port;
    }
    config.app.name = service_name;
    if let Some(pub_certs) = args.pub_certs {
        config.app.pub_certs = Some(pub_certs);
    }
    if let Some(priv_cert) = args.priv_cert {
        config.app.priv_cert = Some(priv_cert);
    }
    if args.debug {
        config.app.debug = true;
    }
    println!("App config {:#?}", config.app);
    Ok(config)
}
