pub mod api;
mod config;

use actix_web::{middleware::Logger, web, App, HttpServer};
use config::Config;

pub async fn init_server() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "info,actix_web=info");
    env_logger::init();

    let local_cfg = Config::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api").configure(api::init_health_handler), // ...so this handles requests for `GET /app/index.html`
            )
            .default_service(web::to(api::not_found))
    })
    .bind(("127.0.0.1", local_cfg.port))?
    .run()
    .await
}
