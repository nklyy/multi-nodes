use actix_web::{http, test, web, App};
use multi_nodes::{
    api::btc::handler::{CreateTxRequest, ToAddresses, Utxo},
    config::Config,
    request,
};
// use multi_nodes::

#[ignore = "comment this when you up your node"]
#[actix_web::test]
async fn create_tx() {
    let request_client = request::RequestClient::new();
    let cfg = Config::init();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(request_client.clone()))
            .app_data(web::Data::new(cfg.bitcoin_rpc_config.clone()))
            .service(web::scope("/api/bitcoin").configure(multi_nodes::api::init_bitcoin_handler)),
    )
    .await;

    let data = CreateTxRequest {
        utxos: Some(vec![Utxo {
            tx_id: Some(
                "989d301c546841d0ac5c8354c7d78079e3603b089682d1639b2ee1c1a8010c6a".to_string(),
            ),
            vout: Some(1),
            amount: Some(0.001),
            pk_script: Some("76a914690cd6356789d30b99063632e0651a8d0c206c7f88ac".to_string()),
        }]),
        to: Some(vec![ToAddresses {
            to_address: Some("mmfbzo2533SFa34ErmYNY4RdVtfw5XYK1u".to_string()),
            amount: Some(0.0001),
        }]),
        change_address: Some("mmfbzo2533SFa34ErmYNY4RdVtfw5XYK1u".to_string()),
    };

    let req = test::TestRequest::post()
        .uri("/api/bitcoin/create-tx")
        .set_json(&data)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::OK);
}
