use std::path::PathBuf;

use config::{Config, Environment, File};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| Settings::get().expect("Config error"));

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub domain: String,
    pub port: u16,
    pub oauth_provider: AuthSettings,
    pub database: DbSettings,
    pub search_engine: SearchSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchSettings {
    pub url: String,
    pub api_key: String,
    pub admin_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbSettings {
    pub database: String,
    pub host: String,
    pub port: u32,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthSettings {
    pub client_id: String,
    pub client_secret: String,
    pub provider: String,
    pub user_info_url: String,
    pub auth_url: String,
    pub token_url: String,
}

impl Settings {
    pub(crate) fn get() -> Result<Self, config::ConfigError> {
        let mut config = Config::builder().add_source(
            Environment::with_prefix("MANUCURE")
                .try_parsing(true)
                .separator("_"),
        );

        let etc_config = PathBuf::from("/etc/manucure/config.toml");
        if etc_config.exists() {
            config = config.add_source(File::from(etc_config));
        }

        let default_config = PathBuf::from("config.toml");
        if default_config.exists() {
            config = config.add_source(File::from(default_config));
        }

        config.build()?.try_deserialize()
    }

    pub fn database_url(&self) -> String {
        format!(
            "postgres://{user}:{pwd}@{host}:{port}/{db}",
            user = self.database.user,
            pwd = self.database.password,
            host = self.database.host,
            port = self.database.port,
            db = self.database.database
        )
    }

    pub fn protocol(&self) -> &str {
        if self.debug {
            "http"
        } else {
            "https"
        }
    }

    pub fn token_url(&self) -> String {
        format!(
            "{}{}",
            self.oauth_provider.provider, self.oauth_provider.token_url
        )
    }

    pub fn auth_url(&self) -> String {
        format!(
            "{}{}",
            self.oauth_provider.provider, self.oauth_provider.auth_url
        )
    }

    pub fn user_info_url(&self) -> String {
        format!(
            "{}{}",
            self.oauth_provider.provider, self.oauth_provider.user_info_url
        )
    }
}
