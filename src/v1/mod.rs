mod voting;

use actix_web::{web, get, HttpResponse, Scope, Responder};
use crate::common::MessageResponse;

pub fn v1() -> Scope {
    web::scope("/v1")
        .service(index)
        .service(voting::voting())
}

#[get("")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(MessageResponse::new(String::from("hello from mixtape api"), true))
}


