use figment::Figment;
use figment::providers::{Env, Format, Toml};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ApiConfig {
    pub host: String,
    pub port: i16,
    pub redis: Redis,
    pub votes: Votes,
}

#[derive(Deserialize, Clone)]
pub struct Votes {
    pub webhook: Webhook,
    pub auth: Auth,
}

#[derive(Deserialize, Clone)]
pub struct Webhook {
    pub url: String,
    pub color: i32,
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
        .merge(Env::raw())
        .select("api")
        .extract()
}

