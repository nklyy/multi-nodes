use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct Segwit {
    #[serde(rename = "type")]
    Type: String,
    active: bool,
    height: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct Taproot {
    #[serde(rename = "type")]
    Type: String,
    active: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct Softforks {
    segwit: Segwit,
    taproot: Taproot,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct BlockchainInfo {
    chain: String,
    blocks: usize,
    headers: usize,
    bestblockhash: String,
    difficulty: usize,
    mediantime: usize,
    verificationprogress: usize,
    initialblockdownload: bool,
    chainwork: String,
    size_on_disk: usize,
    pruned: bool,
    pruneheight: usize,
    automatic_pruning: bool,
    prune_target_size: usize,
    softforks: Softforks,
}
