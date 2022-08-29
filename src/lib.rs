pub mod api;
mod config;
pub mod request;

use actix_web::{middleware::Logger, web, App, HttpServer};
use config::Config;

pub async fn init_server() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "info,actix_web=info");
    env_logger::init();

    let cfg = Config::init();

    let request_client = request::RequestClient::new();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(request_client.clone()))
            .app_data(web::Data::new(cfg.bitcoin_rpc_config.clone()))
            .service(
                web::scope("/api")
                    .configure(api::init_health_handler)
                    .service(web::scope("/bitcoin").configure(api::init_bitcoin_handler)),
            )
            .default_service(web::to(api::not_found))
    })
    .bind(("127.0.0.1", cfg.port))?
    .run()
    .await
}
