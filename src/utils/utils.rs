use crate::error::StabuseError;
use bcrypt::{hash, DEFAULT_COST};
use serde_json::{self, Value};
use std::collections::HashMap;

pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MIN_USERNAME_LENGTH: usize = 3;
const TOKEN_DECIMALS: &[(&str, u32)] = &[("USDC", 6), ("DAI", 18), ("USDT", 6), ("BUSD", 18)];

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

pub fn get_token_decimals(asset: &str) -> Result<u32, StabuseError> {
    TOKEN_DECIMALS
        .iter()
        .find(|(token, _)| *token == asset)
        .map(|(_, decimals)| *decimals)
        .ok_or_else(|| StabuseError::InvalidData(format!("Unsupported token: {}", asset)))
}
