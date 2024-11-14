use std::collections::HashMap;

use crate::{db::migrations::{ADD_ASSET, ADD_NETWORK, GET_ASSETS}, error::StabuseError, types::{self, types::Asset}, utils::{utils::{hashmap_to_json_value, transform_assets_to_uppercase}, validation::validate_assets}};
use serde_json::Value;
use sqlx::PgPool;

use types::types::Network;

pub async fn add_network(pool: &PgPool, network: Network) -> Result<i32, StabuseError> {
    /* TODO
    - Limit to only admins
    */
    validate_assets(&network.supported_assets)?;
    let assets= transform_assets_to_uppercase(&network.supported_assets);
    
    match hashmap_to_json_value(assets) {
        Ok(json_assets) => {
            let id = sqlx::query_scalar(ADD_NETWORK)
                                                .bind(network.chain_id)
                                                .bind(network.name)
                                                .bind(network.rpc)
                                                .bind(json_assets)
                                                .fetch_one(pool)
                                                .await?;
            Ok(id)
        },
        Err(e) => return Err(e),
    }
}

pub async fn add_asset(pool: &PgPool, chain_id:i32, asset: HashMap<String, String>) -> Result<(), StabuseError> {
    validate_assets(&asset)?;
    let assets = transform_assets_to_uppercase(&asset);

    for (key, value) in assets.clone() {
        match hashmap_to_json_value(asset.clone()) {
            Ok(_) => {
                sqlx::query(ADD_ASSET)
                    .bind(key)  
                    .bind(value) 
                    .bind(chain_id) 
                    .execute(pool) 
                    .await?;
            },
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}


pub async fn get_supported_assets(pool: &PgPool, chain_id: i64) -> Result<Vec<Asset>, sqlx::Error> {
    let result: (Value,) = sqlx::query_as(GET_ASSETS)
        .bind(chain_id)
        .fetch_one(pool)
        .await?;

    let assets: Vec<Asset> = serde_json::from_value(result.0)
        .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

    Ok(assets)
}
