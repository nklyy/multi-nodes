use std::collections::HashMap;

use actix_web::{get, http, web, HttpResponse, Responder};
use base64::encode;
use log::error;
use serde::Serialize;
use serde_json::json;

use crate::{
    api::bitcoin::model::BlockchainInfo, config::BitcoinRpcConfig, request::RequestClient,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(status);
}

#[derive(Serialize)]
struct ResponseError {
    message: String,
}

#[get("/status")]
async fn status(
    rq_client: web::Data<RequestClient>,
    btc_rpc_cfg: web::Data<BitcoinRpcConfig>,
) -> impl Responder {
    let mut headers = HashMap::new();
    headers.insert(
        "Content-Type".to_string(),
        "application/x-www-form-urlencoded".to_string(),
    );

    headers.insert(
        "Authorization".to_string(),
        format!(
            "Basic {}",
            encode(format!(
                "{}:{}",
                btc_rpc_cfg.bitcoin_rpc_user, btc_rpc_cfg.bitcoin_rpc_password
            ))
        ),
    );

    let payload = json!({ "jsonrpc": "2.0",  "method": "getblockchaininfo"});

    match rq_client
        .post(&btc_rpc_cfg.bitcoin_rpc_url_one, Some(&headers), &payload)
        .await
    {
        Ok(response) => {
            if response.status() == http::StatusCode::UNAUTHORIZED {
                HttpResponse::Unauthorized().finish()
            } else if response.status() == http::StatusCode::OK {
                match response.json::<BlockchainInfo>().await {
                    Ok(info) => HttpResponse::Ok().json(info),
                    Err(err) => {
                        error!("failed to decode BlockchainInfo, {}", err);
                        HttpResponse::BadRequest().json(ResponseError {
                            message: "failed to decode BlockchainInfo".to_string(),
                        })
                    }
                }
            } else {
                HttpResponse::BadRequest().finish()
            }
        }
        Err(err) => {
            error!("request error: {}", err);
            HttpResponse::RequestTimeout().json(ResponseError {
                message: "failed to do request, something wrong with rpc node".to_string(),
            })
        }
    }
}
