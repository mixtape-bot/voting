use actix_web::{Scope, web, post, HttpRequest, Responder};
use actix_web::http::header::AUTHORIZATION;
use log::error;
use redis_async::resp_array;
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::config::ApiConfig;
use crate::{HttpResponse, Redis};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TopGGVoteType {
    Test,
    Upvote
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct TopGGVote {
    bot: String,
    user: String,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    vote_type: String,
    query: Option<String>,
    is_weekend: bool
}

fn top_gg_vote_key(bot: &String, user: &String) -> String {
    format!("votes.top_gg:{}.{}", bot, user)
}

pub fn voting() -> Scope {
    web::scope("/voting")
        .service(post_top_gg_vote)
}

#[post("/top-gg")]
async fn post_top_gg_vote(req: HttpRequest, config: web::Data<ApiConfig>, redis: web::Data<Redis>, payload: web::Json<TopGGVote>) -> impl Responder {
    let auth = match req.headers().get(AUTHORIZATION) {
        Some(value) => match value.to_str() {
            Err(e) => {
                error!("Error occurred while validating top.gg auth: {}", e);

                return HttpResponse::BadRequest()
                    .json(json!({ "message": "Unable to validate authorization", "success": false }))
            },
            Ok(value) => value.to_string()
        },
        _ => return HttpResponse::Forbidden()
            .json(json!({ "message": "No authorization was provided", "success": false }))
    };

    /* check if their authentication is valid */
    if config.auth.top_gg != auth {
        return HttpResponse::Unauthorized()
            .json(json!({ "message": "Invalid authorization was provided", "success": false }))
    }

    /* store the vote in redis */
    let payload_json = serde_json::to_string(&*payload).unwrap();
    redis
        .get()
        .send_and_forget(resp_array!["SET", top_gg_vote_key(&payload.bot, &payload.user), payload_json, "EX", "720"]);

    /* respond */
    HttpResponse::Ok().json(json!({ "message": "thanks for that", "success": true }))
}
