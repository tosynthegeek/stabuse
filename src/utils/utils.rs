use crate::{error::StabuseError, network::network::is_asset_supported_on_network};
use bcrypt::{hash, DEFAULT_COST};
use serde_json::{self, Value};
use sqlx::PgPool;
use std::collections::HashMap;

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
    let password_hash =
        hash(password, DEFAULT_COST).map_err(|e| StabuseError::HashError(e.to_string()))?;
    Ok(password_hash)
}

pub async fn validate_supported_networks(
    pool: &PgPool,
    supported_assets: &serde_json::Value,
) -> Result<(), StabuseError> {
    if let Some(assets_map) = supported_assets.as_object() {
        for (chain_id, assets) in assets_map {
            let chain_id: i64 = chain_id.parse().map_err(|_| {
                StabuseError::InvalidData(format!("Invalid chain ID: {}", chain_id))
            })?;

            if let Some(assets_array) = assets.as_array() {
                for asset_value in assets_array {
                    if let Some(asset) = asset_value.as_str() {
                        if !is_asset_supported_on_network(pool, chain_id, &asset.to_uppercase())
                            .await?
                        {
                            return Err(StabuseError::AssetNotSupportedonNetwork(format!(
                                "Network {} does not support asset {}",
                                chain_id, asset
                            )));
                        }
                    } else {
                        return Err(StabuseError::InvalidData(format!(
                            "Asset is not a valid string: {:?}",
                            asset_value
                        )));
                    }
                }
            } else {
                return Err(StabuseError::InvalidData(format!(
                    "Assets for chain ID {} must be an array",
                    chain_id
                )));
            }
        }
    } else {
        return Err(StabuseError::InvalidData(
            "Supported assets must be a JSON object".to_string(),
        ));
    }
    Ok(())
}

pub async fn validate_supported_assets(
    pool: &PgPool,
    chain_id: i64,
    supported_assets: Vec<String>,
) -> Result<(), StabuseError> {
    for asset in supported_assets {
        if !is_asset_supported_on_network(pool, chain_id, &asset.to_uppercase()).await? {
            return Err(StabuseError::AssetNotSupportedonNetwork(format!(
                "Network {} does not support asset {}",
                chain_id, asset
            )));
        }
    }

    Ok(())
}
