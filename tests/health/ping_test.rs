use actix_web::{
    test,
    web::{self, Bytes},
    App,
};

#[actix_web::test]
async fn ping() {
    let app = test::init_service(
        App::new().service(web::scope("/api").configure(multi_nodes::api::init_health_handler)),
    )
    .await;

    let req = test::TestRequest::get().uri("/api/ping").to_request();

    let resp = test::call_and_read_body(&app, req).await;

    assert_eq!(resp, Bytes::from_static(b"{\"message\":\"pong\"}"));
}
