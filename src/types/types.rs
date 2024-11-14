use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Network {
    pub chain_id: i32,
    pub name: String,
    pub explorer: String,
    pub rpc: String,
    pub supported_assets: HashMap<String, String>
}

pub struct Client {
    pub username: String,
    pub supported_networks: HashMap<i32, String> // Map the network chain ID to the client address
}

#[derive(Deserialize, Serialize)]
pub struct Asset {
    pub asset_name: String,
    pub symbol: String,
}