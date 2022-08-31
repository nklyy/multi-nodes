use std::collections::HashMap;

use actix_web::{get, http, post, web, HttpResponse, Responder};
use base64::encode;
use log::error;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use validator::Validate;

use crate::{
    api::bitcoin::model::{BlockchainInfo, CreateTx},
    config::BitcoinRpcConfig,
    request::RequestClient,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(status);
    cfg.service(create_tx);
}

#[derive(Serialize)]
struct ErrorResponse {
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
                HttpResponse::Unauthorized().json(ErrorResponse {
                    message: "Unauthorized".to_string(),
                })
            } else {
                match response.json::<BlockchainInfo>().await {
                    Ok(info) => {
                        if let Some(err) = info.error {
                            HttpResponse::BadRequest().json(ErrorResponse {
                                message: err.message,
                            })
                        } else {
                            HttpResponse::Ok().json(info)
                        }
                    }
                    Err(err) => {
                        error!("failed to decode BlockchainInfo, {}", err);
                        HttpResponse::BadRequest().json(ErrorResponse {
                            message: "failed to decode response".to_string(),
                        })
                    }
                }
            }
        }
        Err(err) => {
            error!("request error: {}", err);
            HttpResponse::RequestTimeout().json(ErrorResponse {
                message: "failed to do request, something wrong with rpc node".to_string(),
            })
        }
    }
}

// ^(bc1|[13])[a-zA-HJ-NP-Z0-9]{25,39}$ - mainnet
// ^(tb1|[2nm]|bcrt)[a-zA-HJ-NP-Z0-9]{25,40}$ - testnet
lazy_static::lazy_static! {
    static ref RE_BITCOIN_ADDRESS: Regex =
        Regex::new(r"^(tb1|[2nm]|bcrt)[a-zA-HJ-NP-Z0-9]{25,40}$").unwrap();
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Utxo {
    #[validate(required, length(min = 1, message = "cannot be empty"))]
    pub tx_id: Option<String>,

    #[validate(required, range(min = 0, message = "cannot be empty"))]
    pub vout: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ToAddresses {
    #[validate(
        required,
        regex(path = "RE_BITCOIN_ADDRESS", message = "invalid Bitcoin address")
    )]
    pub to_address: Option<String>,

    #[validate(required, range(min = 0.0, message = "cannot be empty"))]
    pub amount: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTxRequest {
    // #[validate]
    #[validate(required, length(min = 1, message = "cannot be empty"))]
    pub utxos: Option<Vec<Utxo>>,

    #[validate]
    #[validate(required, length(min = 1, message = "cannot be empty"))]
    pub to: Option<Vec<ToAddresses>>,
}

#[post("/create-tx")]
async fn create_tx(
    json: web::Json<CreateTxRequest>,
    rq_client: web::Data<RequestClient>,
    btc_rpc_cfg: web::Data<BitcoinRpcConfig>,
) -> impl Responder {
    match json.validate() {
        Ok(_) => {
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "application/json".to_string());
            headers.insert("Accept".to_string(), "application/json".to_string());

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

            let mut payload_to_addresses: Vec<HashMap<String, f64>> = Vec::new();
            json.to.as_ref().unwrap().iter().for_each(|to| {
                let mut r: HashMap<String, f64> = HashMap::new();
                r.insert(
                    to.to_address.as_ref().unwrap().to_string(),
                    to.amount.unwrap(),
                );

                payload_to_addresses.push(r);
            });

            let mut payload_utxos: Vec<Value> = Vec::new();
            json.utxos.as_ref().unwrap().iter().for_each(|utxo| {
                payload_utxos.push(
                    json!({"txid": utxo.tx_id.as_ref().unwrap(), "vout": utxo.vout.unwrap()}),
                );
            });

            let payload = json!({ "jsonrpc": "2.0",  "method": "createrawtransaction", "params": [payload_utxos, payload_to_addresses]});

            match rq_client
                .post(&btc_rpc_cfg.bitcoin_rpc_url_one, Some(&headers), &payload)
                .await
            {
                Ok(response) => {
                    if response.status() == http::StatusCode::UNAUTHORIZED {
                        HttpResponse::Unauthorized().json(ErrorResponse {
                            message: "Unauthorized".to_string(),
                        })
                    } else {
                        match response.json::<CreateTx>().await {
                            Ok(info) => {
                                if let Some(err) = info.error {
                                    HttpResponse::BadRequest().json(ErrorResponse {
                                        message: err.message,
                                    })
                                } else {
                                    HttpResponse::Ok().json(info)
                                }
                            }
                            Err(err) => {
                                error!("failed to decode BlockchainInfo, {}", err);
                                HttpResponse::BadRequest().json(ErrorResponse {
                                    message: "failed to decode response".to_string(),
                                })
                            }
                        }
                    }
                }
                Err(err) => {
                    error!("request error: {}", err);
                    HttpResponse::RequestTimeout().json(ErrorResponse {
                        message: "failed to do request, something wrong with rpc node".to_string(),
                    })
                }
            }
        }
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}
