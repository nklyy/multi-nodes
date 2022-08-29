use serde::{Deserialize, Serialize};

// #[derive(Deserialize)]
// #[serde(rename_all = "snake_case")]
// struct Segwit {
//     #[serde(rename = "type")]
//     Type: String,
//     active: bool,
//     height: usize,
// }

// #[derive(Deserialize)]
// #[serde(rename_all = "snake_case")]
// struct Taproot {
//     #[serde(rename = "type")]
//     Type: String,
//     active: bool,
// }

// #[derive(Deserialize)]
// #[serde(rename_all = "snake_case")]
// struct Softforks {
//     segwit: Segwit,
//     taproot: Taproot,
// }

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
struct BlockchainInfoResult {
    chain: String,
    blocks: usize,
    headers: usize,
    bestblockhash: String,
    difficulty: usize,
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
#[serde(rename_all = "snake_case")]
struct RPCError {
    code: isize,
    message: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct BlockchainInfo {
    result: Option<BlockchainInfoResult>,
    error: Option<RPCError>,
}
