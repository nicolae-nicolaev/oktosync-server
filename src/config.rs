use config::{Config, ConfigError, Environment, File};
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub database_url: String,
}

pub fn load_config() -> Result<Settings, ConfigError> {
    dotenv().ok();

    let builder = Config::builder()
        .add_source(File::with_name("config/default").required(false))
        .add_source(Environment::default().separator("_"));

    builder.build()?.try_deserialize()
}
