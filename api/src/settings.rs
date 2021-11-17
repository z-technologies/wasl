use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

use std::env;

#[derive(Debug, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u16,
}

impl Server {
    pub fn endpoint(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub backend: String,
    pub address: String,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl Database {
    pub fn url(&self) -> String {
        format!(
            "{}://{}:{}@{}/{}",
            self.backend,
            self.username,
            self.password,
            self.address,
            self.database
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Security {
    pub private_key: String,
    pub public_key: String,
    pub token_expiration_seconds: i64,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
    pub security: Security,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::default();

        let prefix =
            env::var("CONFIG_PREFIX").unwrap_or_else(|_| "config".into());
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        // default config
        s.merge(File::with_name(&format!("{}/default", prefix)))?;

        // run environment config
        s.merge(
            File::with_name(&format!("{}/{}", prefix, env)).required(false),
        )?;

        // environment add-ins
        s.merge(Environment::with_prefix("wasl"))?;

        // build config
        s.try_into()
    }
}
