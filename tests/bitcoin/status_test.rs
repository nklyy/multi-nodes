use actix_web::{http, test, web, App};
use rpc_api::{config::Config, request};

#[ignore = "comment this when you up your node"]
#[actix_web::test]
async fn status() {
    let request_client = request::RequestClient::new();
    let cfg = Config::init();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(request_client.clone()))
            .app_data(web::Data::new(cfg.bitcoin_rpc_config.clone()))
            .service(web::scope("/api/bitcoin").configure(rpc_api::api::init_bitcoin_handler)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/bitcoin/status")
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);
}
