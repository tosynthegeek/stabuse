use crate::error::StabuseError;
use bcrypt::{hash, DEFAULT_COST};
use serde_json::{self, Value};
use std::{collections::HashMap, fs};

pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MIN_USERNAME_LENGTH: usize = 3;

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

pub fn get_network_and_asset_address(
    asset: &str,
    chain_id: u64,
) -> Result<(String, String), StabuseError> {
    let data = fs::read_to_string("config/assets.json")?;

    let assets: Value = serde_json::from_str(&data)?;

    if let Some(network_data) = assets.get(chain_id.to_string()) {
        let network_name = network_data
            .get("network")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                StabuseError::AssetNotSupportedonNetwork(format!(
                    "Network name not found for chain ID {}",
                    chain_id
                ))
            })?;

        let asset_address = network_data
            .get(asset)
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                StabuseError::AssetNotSupportedonNetwork(format!(
                    "Asset {} not found on network {}",
                    asset, chain_id
                ))
            })?;

        Ok((network_name.to_string(), asset_address.to_string()))
    } else {
        Err(StabuseError::AssetNotSupportedonNetwork(format!(
            "Chain ID {} not found in configuration",
            chain_id
        )))
    }
}
