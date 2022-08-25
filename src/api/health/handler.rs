use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
}

#[derive(Serialize)]
struct CheckResponse {
    message: String,
}

#[get("/ping")]
async fn ping() -> impl Responder {
    let resp = CheckResponse {
        message: String::from("pong"),
    };

    HttpResponse::Ok().json(resp)
}