use figment::Figment;
use figment::providers::{Env, Format, Toml};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct ApiConfig {
    pub host: String,
    pub port: i16,
    pub redis: Redis,
    pub webhook: Webhook,
    pub auth: Auth,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Webhook {
    pub url: String,
    pub color: i32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Auth {
    pub top_gg: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Redis {
    pub host: String,
    pub port: i16,
}

pub fn load_config() -> Result<ApiConfig, figment::Error> {
    Figment::new()
        .merge(Toml::file("voting.toml"))
        .merge(Env::prefixed("API_"))
        .extract()
}

