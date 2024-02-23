use config::Config;
use secrecy::{ExposeSecret, Secret, SecretString};
use serde::Deserialize;

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
}

impl DatabaseSettings {
    pub fn url(&self) -> SecretString {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        ))
    }

    pub fn url_without_db(&self) -> SecretString {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
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
