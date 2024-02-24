use config::Config;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use sqlx::ConnectOptions;
use tracing::log::LevelFilter;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub app: AppSettings,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub host: String,
    pub port: u16,
    pub name: String,
    pub ssl: bool,
}

impl DatabaseSettings {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db()
            .database(&self.name)
            .log_statements(LevelFilter::Trace)
    }
}

#[derive(Deserialize)]
pub struct AppSettings {
    pub port: u16,
    pub host: String,
    pub log: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let config = Config::builder()
        .add_source(
            config::Environment::default()
                .try_parsing(true)
                .separator("_"),
        )
        .build()
        .unwrap();

    config.try_deserialize()
}
