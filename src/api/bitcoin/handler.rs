use actix_web::{get, web, HttpResponse, Responder};

use crate::request::RequestClient;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(status);
}

#[get("/status")]
async fn status(rq_client: web::Data<RequestClient>) -> impl Responder {
    match rq_client
        .post("http://127.0.0.1:8080", &Default::default())
        .await
    {
        Ok(response) => HttpResponse::Ok().body(format!("OK, status: {}", response.status())),
        Err(err) => HttpResponse::BadRequest().body(format!("{:?}", err)),
    }
}
