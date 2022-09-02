use crate::database::DatabaseConfig;
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
    #[clap(long, default_value = "INFO", env = "LOG_LEVEL")]
    log_level: LogLevel,
    #[clap(long, default_value = "8888", env = "PORT")]
    /// The port to listen on
    port: u16,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub db: DatabaseConfig,
    #[serde(skip)]
    pub app: AppConfig,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub name: String,
    pub log_level: LogLevel,
    pub port: u16,
}
pub fn load_config(service_name: String) -> Result<Config> {
    let args: CliArgument = CliArgument::parse();

    println!("Working directory {}", current_dir()?.display());
    println!("Loading config from {}", args.config.display());
    let config = std::fs::read_to_string(&args.config)?;
    let mut config: Config = serde_json::from_str(&config)?;
    config.app.log_level = args.log_level;
    config.app.port = args.port;
    config.app.name = service_name;
    Ok(config)
}
