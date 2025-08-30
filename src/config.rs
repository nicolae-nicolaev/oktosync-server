use std::env;

use config::{Config, ConfigError, Environment, File};
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: None,
            port: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: Option<String>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self { url: None }
    }
}

pub fn load_config() -> Result<Settings, ConfigError> {
    if let Err(err) = dotenv() {
        log::warn!("⚠️ Failed to load .env file: {err}");
    }

    let builder = Config::builder()
        .add_source(File::with_name("config/default").required(false))
        .add_source(
            Environment::with_prefix("OKTOSYNC")
                .prefix_separator("_")
                .separator("__")
                .try_parsing(true)
                .ignore_empty(true),
        );

    builder.build()?.try_deserialize()
}

pub fn resolve_database_url(settings: &Settings) -> Result<String, String> {
    if let Ok(url) = env::var("DATABASE_URL") {
        return Ok(url);
    }
    if let Some(url) = &settings.database.url {
        return Ok(url.clone());
    }

    Err(
        "No database URL found. Set DATABASE_URL or OKTOSYNC_DATABASE__URL environment variables."
            .into(),
    )
}
