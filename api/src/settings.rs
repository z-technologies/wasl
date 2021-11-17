use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

use std::env;

#[derive(Debug, Deserialize)]
struct Server {
    address: String,
    port: u16,
}

impl Server {
    fn endpoint(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

#[derive(Debug, Deserialize)]
struct Database {
    backend: String,
    address: String,
    username: String,
    password: String,
    database: String,
}

impl Database {
    fn url(&self) -> String {
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
struct Security {
    private_key: String,
    public_key: String,
    token_expiration_seconds: u64,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    server: Server,
    database: Database,
    security: Security,
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
