use std::collections::HashMap;

use actix_web::{get, http, post, web, HttpResponse, Responder};
use base64::encode;
use log::error;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use crate::{
    api::btc::{
        model::{BlockchainInfo, FeeRate, SendTx, SignTx},
        service::create_transaction,
    },
    config::BitcoinRpcConfig,
    request::RequestClient,
};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(status);
    cfg.service(create_tx);
    cfg.service(sign_tx);
    cfg.service(send_tx);
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Serialize)]
struct OkResponse {
    result: String,
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
    pub vout: Option<u32>,

    #[validate(required, range(min = 0.00000001, message = "cannot be empty"))]
    pub amount: Option<f64>,

    #[validate(required, length(min = 1, message = "cannot be empty"))]
    pub pk_script: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ToAddresses {
    #[validate(
        required,
        regex(path = "RE_BITCOIN_ADDRESS", message = "invalid Bitcoin address")
    )]
    pub to_address: Option<String>,

    #[validate(required, range(min = 0.00000001, message = "cannot be empty"))]
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

    #[validate(required, length(min = 1, message = "cannot be empty"))]
    pub change_address: Option<String>,
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

            let payload = json!({ "jsonrpc": "2.0",  "method": "estimatesmartfee", "params": [4]});

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
                        match response.json::<FeeRate>().await {
                            Ok(fee_rate) => {
                                if let Some(err) = fee_rate.error {
                                    HttpResponse::BadRequest().json(ErrorResponse {
                                        message: err.message,
                                    })
                                } else {
                                    match create_transaction(
                                        json.utxos.as_ref().unwrap(),
                                        json.to.as_ref().unwrap(),
                                        json.change_address.as_ref().unwrap(),
                                        fee_rate.result.unwrap().feerate,
                                    ) {
                                        Ok(hex_tx) => {
                                            return HttpResponse::Ok()
                                                .json(OkResponse { result: hex_tx })
                                        }
                                        Err(err) => {
                                            return HttpResponse::BadRequest().json(ErrorResponse {
                                                message: err.to_string(),
                                            })
                                        }
                                    };
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

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SignTxRequest {
    #[validate(required, length(min = 1, message = "cannot be empty"))]
    pub raw_tx: Option<String>,

    #[validate(required, length(min = 1, message = "cannot be empty"))]
    pub private_key: Option<String>,
}

#[post("/sign-tx")]
async fn sign_tx(
    json: web::Json<SignTxRequest>,
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

            let payload = json!({ "jsonrpc": "2.0",  "method": "signrawtransactionwithkey", "params": [json.raw_tx.as_ref().unwrap(), [json.private_key.as_ref().unwrap()]]});

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
                        match response.json::<SignTx>().await {
                            Ok(signed_tx) => {
                                if let Some(err) = signed_tx.error {
                                    HttpResponse::BadRequest().json(ErrorResponse {
                                        message: err.message,
                                    })
                                } else {
                                    HttpResponse::Ok().json(signed_tx)
                                }
                            }
                            Err(err) => {
                                error!("failed to sign tx, {}", err);
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

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SendTxRequest {
    #[validate(required, length(min = 1, message = "cannot be empty"))]
    pub signed_tx: Option<String>,
}

#[post("/send-tx")]
async fn send_tx(
    json: web::Json<SendTxRequest>,
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

            let payload = json!({ "jsonrpc": "2.0",  "method": "sendrawtransaction", "params": [json.signed_tx.as_ref().unwrap()]});

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
                        match response.json::<SendTx>().await {
                            Ok(sended_tx) => {
                                if let Some(err) = sended_tx.error {
                                    HttpResponse::BadRequest().json(ErrorResponse {
                                        message: err.message,
                                    })
                                } else {
                                    HttpResponse::Ok().json(sended_tx)
                                }
                            }
                            Err(err) => {
                                error!("failed to sign tx, {}", err);
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
