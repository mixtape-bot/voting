mod common;
mod v1;
mod config;

use actix_web::{HttpResponse, Responder, middleware, web};
use log::info;
use serde_json::json;
use redis_async::client::PairedConnection;
use crate::config::{load_config};

#[derive(Clone)]
pub struct Redis {
    conn: PairedConnection
}

impl Redis {
    async fn new(uri: impl Into<String>) -> Result<Self, redis_async::error::Error> {
        let redis = redis_async::client::paired_connect(uri).await?;

        Ok(Self {
            conn: redis
        })
    }

    pub fn get(&self) -> PairedConnection {
        return self.conn.clone()
    }
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({ "versioned_routes": [ "/v1" ], "success": true }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    env_logger::init();

    /* load config */
    let config = match load_config() {
        Ok(config) => config,
        Err(e) => panic!("{}", e)
    };

    info!("Loaded config");

    /* connect to redis */
    let redis = Redis::new(format!("{}:{}", config.redis.host, config.redis.port))
        .await
        .expect("Unable to connect to redis.");

    info!("Connected to redis");

    /* start the server */
    let addr = format!("{}:{}", config.host, config.port);
    HttpServer::new(move || App::new()
        .app_data(web::Data::new(config.clone()))
        .app_data(web::Data::new(redis.clone()))
        .wrap(middleware::Logger::default())
        .service(index)
        .service(v1::v1())
    )
        .bind(addr)?
        .run()
        .await
}
