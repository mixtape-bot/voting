use actix_web::{Scope, web, post, HttpRequest, Responder};
use actix_web::http::header::AUTHORIZATION;
use log::{error, info};
use redis_async::resp_array;
use serde::{Serialize, Deserialize};
use crate::common::MessageResponse;
use crate::config::ApiConfig;
use crate::{HttpResponse, Redis};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TopGGVoteType {
    Test,
    Upvote
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct TopGGVote {
    bot: String,
    user: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    vote_type: String,
    query: Option<String>,
    is_weekend: bool
}

pub fn voting() -> Scope {
    web::scope("/voting")
        .service(get_top_gg_vote)
        .service(post_top_gg_vote)
}

#[get("/top-gg")]
async fn get_top_gg_vote() {

}

#[post("/top-gg")]
async fn post_top_gg_vote(req: HttpRequest, config: web::Data<ApiConfig>, redis: web::Data<Redis>, payload: web::Json<TopGGVote>) -> impl Responder {
    let auth = match req.headers().get(AUTHORIZATION) {
        Some(value) => match value.to_str() {
            Err(e) => {
                error!("Error occurred while validating top.gg auth: {}", e);

                return HttpResponse::BadRequest().json(MessageResponse::new("Unable to validate authorization".into(), false))
            },
            Ok(value) => value.to_string()
        },
        _ => return HttpResponse::Forbidden().json(MessageResponse::new("No authorization was provided".into(), false))
    };

    /* check if their authentication is valid */
    if config.auth.top_gg != auth {
        return HttpResponse::Unauthorized().json(MessageResponse::new("Invalid authorization was provided".into(), false))
    }

    /* store the vote in redis */
    let payload_json = serde_json::to_string(&*payload).unwrap();
    let conn = redis.get();
    conn.send_and_forget(resp_array!["SET", format!("votes.top_gg:{}.{}", payload.bot, payload.user), payload_json, "EX", "720"]);

    /* respond */
    HttpResponse::Ok().json(MessageResponse::new("thanks for that".to_string(), true))
}
