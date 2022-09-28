use serde::{Deserialize, Serialize};

// #[derive(Deserialize)]
// struct Segwit {
//     #[serde(rename = "type")]
//     Type: String,
//     active: bool,
//     height: usize,
// }

// #[derive(Deserialize)]
// struct Taproot {
//     #[serde(rename = "type")]
//     Type: String,
//     active: bool,
// }

// #[derive(Deserialize)]
// struct Softforks {
//     segwit: Segwit,
//     taproot: Taproot,
// }

#[derive(Deserialize, Serialize)]
pub struct RPCError {
    pub code: isize,
    pub message: String,
}

#[derive(Deserialize, Serialize)]
struct BlockchainInfoResult {
    chain: String,
    blocks: usize,
    headers: usize,
    bestblockhash: String,
    // difficulty: usize,
    time: usize,
    mediantime: usize,
    verificationprogress: f64,
    initialblockdownload: bool,
    chainwork: String,
    size_on_disk: usize,
    pruned: bool,
    warnings: String,
}
#[derive(Deserialize, Serialize)]
pub struct BlockchainInfo {
    result: Option<BlockchainInfoResult>,
    pub error: Option<RPCError>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateTx {
    result: Option<String>,
    pub error: Option<RPCError>,
}

#[derive(Deserialize, Serialize)]
pub struct FeeRateResult {
    pub feerate: f64,
    pub blocks: usize,
}

#[derive(Deserialize, Serialize)]
pub struct FeeRate {
    pub result: Option<FeeRateResult>,
    pub error: Option<RPCError>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignTxResultErrors {
    pub txid: String,
    pub vout: usize,
    pub script_sig: String,
    pub sequence: usize,
    pub error: String,
}

#[derive(Deserialize, Serialize)]
pub struct SignTxResult {
    pub hex: String,
    pub complete: bool,
    pub errors: Option<Vec<SignTxResultErrors>>,
}

#[derive(Deserialize, Serialize)]
pub struct SignTx {
    pub result: Option<SignTxResult>,
    pub error: Option<RPCError>,
}

#[derive(Deserialize, Serialize)]
pub struct SendTx {
    pub result: Option<String>,
    pub error: Option<RPCError>,
}
