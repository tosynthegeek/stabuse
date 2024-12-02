use sqlx::PgPool;
use std::collections::HashMap;

use crate::{error::StabuseError, network::network::is_asset_supported_on_network};

pub fn validate_assets(assets: &HashMap<String, String>) -> Result<(), StabuseError> {
    for (ticker, address) in assets {
        if ticker == "" {
            return Err(StabuseError::InvalidAssetFormat(format!(
                "Ticker cannot be empty"
            )));
        }
        if address.len() != 42 || !address.starts_with("0x") {
            return Err(StabuseError::InvalidAssetFormat(format!(
                "Invalid address format for asset {}: {}",
                ticker, address
            )));
        }
    }
    Ok(())
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
