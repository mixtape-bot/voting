use figment::Figment;
use figment::providers::{Format, Toml};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ApiConfig {
    pub host: String,
    pub port: i16,
    pub auth: Auth,
    pub redis: Redis,
}

#[derive(Deserialize, Clone)]
pub struct Auth {
    pub top_gg: String,
}

#[derive(Deserialize, Clone)]
pub struct Redis {
    pub host: String,
    pub port: i16,
}

pub fn load_config() -> Result<ApiConfig, figment::Error> {
    Figment::new()
        .merge(Toml::file("api.toml").nested())
        .select("api")
        .extract()
}

