use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum AppEnv {
    Production,
    Development,
    Staging
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

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub env: AppEnv,
    pub host: String,
    pub port: u16,
    pub loki: Option<LokiConfig>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            env: AppEnv::Development,
            host: "0.0.0.0".to_string(),
            port: 8080,
            loki: None
        }
    } 
}
 
impl AppConfig {
    pub fn from_env() -> Self {
        let source = Environment::default().separator("_");
        Config::builder()
            .add_source(source).build().unwrap()
            .try_deserialize().unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct LokiConfig {
    pub host: String,
    pub port: u16
}

impl LokiConfig {
    pub fn get_url(&self) -> String {
        format!{"http://{}:{}", self.host, self.port}
    }
}
