use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct NodeInfo {
    pub chain: String,
}
