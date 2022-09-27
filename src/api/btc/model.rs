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
pub struct RPCError {
    pub code: isize,
    pub message: String,
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
    pub feerate: Option<f64>,
    pub blocks: Option<usize>,
}

#[derive(Deserialize, Serialize)]
pub struct FeeRate {
    pub result: Option<FeeRateResult>,
    pub error: Option<RPCError>,
}
