use actix_web::HttpResponse;
use serde::Serialize;

pub mod btc;
pub mod health;

pub use btc::handler::init as init_bitcoin_handler;
pub use health::handler::init as init_health_handler;

#[derive(Serialize)]
struct NotFoundResponse {
    error: String,
}

pub async fn not_found() -> HttpResponse {
    let resp = NotFoundResponse {
        error: String::from("Page NotFound"),
    };

    HttpResponse::NotFound().json(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http;

    #[actix_web::test]
    async fn test_not_found_ok() {
        let resp = not_found().await;
        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }
}
