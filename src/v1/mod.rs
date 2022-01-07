mod voting;

use actix_web::{web, get, HttpResponse, Scope, Responder};
use serde_json::json;

pub fn v1() -> Scope {
    web::scope("/v1")
        .service(index)
        .service(voting::voting())
}

#[get("")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(json!({ "message": "hello from mixtape's voting api", "success": true }))
}
