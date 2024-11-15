use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Actor {
    Admin,
    Merchant
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Network {
    pub chain_id: i64,
    pub name: String,
    pub explorer: String,
    pub rpc: String,
    pub supported_assets: HashMap<String, String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkDB {
    pub id: i32,
    pub chain_id: i64,
    pub name: String,
    pub rpc_url: String,
    pub supported_assets: HashMap<String, String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

pub struct Merchant {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub supported_networks: HashMap<i32, String> // Map the network chain ID to the client address
}

pub type Asset = HashMap<String, String>;

#[derive(Deserialize)]
pub struct AddAssetRequest {
    pub chain_id: i32,
    pub assets: HashMap<String, String>
}