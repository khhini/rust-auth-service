use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum AppEnv {
    Production,
    Development,
    Staging,
}

impl AsRef<str> for AppEnv {
    fn as_ref(&self) -> &str {
        match self {
            AppEnv::Production => "production",
            AppEnv::Development => "development",
            AppEnv::Staging => "staging",
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub env: AppEnv,
    pub host: String,
    pub port: u16,
    pub loki: LokiConfig,
    pub database: DatabaseConfig,
    pub pwd: PwdConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            env: AppEnv::Development,
            host: "0.0.0.0".to_string(),
            port: 8080,
            loki: LokiConfig::default(),
            database: DatabaseConfig::default(),
            pwd: PwdConfig::default(),
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        let source = Environment::default().separator("_");
        Config::builder()
            .add_source(source)
            .build()
            .unwrap()
            .try_deserialize()
            .unwrap()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct LokiConfig {
    pub host: String,
    pub port: u16,
}

impl LokiConfig {
    pub fn get_url(&self) -> String {
        format! {"http://{}:{}", self.host, self.port}
    }
}

impl Default for LokiConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 3100,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    url: String,
}

impl DatabaseConfig {
    pub fn new(url: String) -> Self {
        Self { url }
    }
    pub fn get_connection_string(&self) -> String {
        self.url.clone()
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://postgres:password@localhost:5432/postgres".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PwdConfig {
    pub secret: String,
}

impl Default for PwdConfig {
    fn default() -> Self {
        Self {
            secret: "SuperDuperSecret".to_string(),
        }
    }
}

pub fn get_config_from_env() -> AppConfig {
    AppConfig::from_env()
}

pub fn get_default_config() -> AppConfig {
    AppConfig::default()
}
