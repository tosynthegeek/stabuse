use crate::error::StabuseError;
use alloy::hex;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use reqwest::Client;
use serde_json::{self, Value};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::env;

pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MIN_USERNAME_LENGTH: usize = 3;
const TOKEN_DECIMALS: &[(&str, u8)] = &[("USDC", 6), ("DAI", 18), ("USDT", 6), ("BUSD", 18)];

pub fn transform_assets_to_uppercase(assets: &HashMap<String, String>) -> HashMap<String, String> {
    assets
        .iter()
        .map(|(ticker, address)| (ticker.to_uppercase(), address.clone()))
        .collect()
}

pub fn hashmap_to_json_value(map: HashMap<String, String>) -> Result<Value, StabuseError> {
    let value =
        serde_json::to_value(map).map_err(|err| StabuseError::SerdeError(err.to_string()))?;

    Ok(value)
}

pub fn hash_password(password: &str) -> Result<String, StabuseError> {
    let password_hash = hash(password, DEFAULT_COST).map_err(|e| StabuseError::HashError(e))?;
    Ok(password_hash)
}

pub fn get_token_decimals(asset: &str) -> Result<u8, StabuseError> {
    TOKEN_DECIMALS
        .iter()
        .find(|(token, _)| *token == asset)
        .map(|(_, decimals)| *decimals)
        .ok_or_else(|| StabuseError::InvalidData(format!("Unsupported token: {}", asset)))
}

pub fn get_solana_network_identifier(rpc_url: &str) -> Result<i64, StabuseError> {
    match rpc_url {
        url if url.contains("mainnet") => {
            return Ok(101);
        }
        url if url.contains("devnet") => {
            return Ok(102);
        }
        url if url.contains("testnet") => {
            return Ok(103);
        }
        _ => {
            return Err(StabuseError::InvalidData(format!(
                "RPC URL {} not recognized",
                rpc_url
            )));
        }
    };
}

pub fn generate_webhook_url(merchant_id: i32, user_address: &str, amount: u64) -> (String, String) {
    let base_webhook_url = env::var("WEBHOOK_BASE_URL").expect("WEBHOOK_BASE_URL must be set");
    let timestamp = Utc::now().to_rfc3339();

    let data = format!("{}:{}:{}:{}", merchant_id, user_address, amount, timestamp);
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let hash_hex = hex::encode(result);

    let webhook_url = format!("{}/{}", base_webhook_url, hash_hex);

    (webhook_url, timestamp)
}

pub async fn send_webhook_notification(webhook_url: &str, data: &str) -> Result<(), StabuseError> {
    let client = Client::new();
    let res = client
        .post(webhook_url)
        .header("Content-Type", "application/json")
        .body(data.to_string())
        .send()
        .await
        .map_err(|e| StabuseError::Internal(format!("Webhook failed: {}", e)))?;

    if res.status().is_success() {
        tracing::info!("Webhook delivered successfully to {}", webhook_url);
    } else {
        tracing::info!("Message published successfully to webhook: {}", webhook_url);
    }

    Ok(())
}
