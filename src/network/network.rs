use std::collections::HashMap;

use crate::{
    db::migrations::networks::{
        insert_and_update_networks::{ADD_ASSET, ADD_NETWORK},
        select_queries::{
            CHECK_NETWORK_SUPPORTED_ASSET, GET_ALL_NETWORKS, GET_NETWORK, GET_NETWORK_ASSETS,
        },
    },
    error::StabuseError,
    types::{
        self,
        types::{Asset, NetworkDB},
    },
    utils::{
        utils::{hashmap_to_json_value, transform_assets_to_uppercase},
        validation::domain_validation::validate_assets,
    },
};
use serde_json::Value;
use sqlx::{PgPool, Row};

use types::types::Network;

pub async fn add_network(
    pool: &PgPool,
    admin_username: &str,
    network: Network,
) -> Result<i32, StabuseError> {
    /* TODO
    - Limit to only admins
    */
    validate_assets(&network.supported_assets)?;
    let assets = transform_assets_to_uppercase(&network.supported_assets);

    match hashmap_to_json_value(assets) {
        Ok(json_assets) => {
            let id = sqlx::query_scalar(ADD_NETWORK)
                .bind(network.chain_id)
                .bind(network.name)
                .bind(network.rpc)
                .bind(json_assets)
                .bind(admin_username)
                .fetch_one(pool)
                .await?;
            Ok(id)
        }
        Err(e) => return Err(e),
    }
}

pub async fn add_asset_to_network(
    pool: &PgPool,
    admin_username: &str,
    chain_id: i32,
    asset: HashMap<String, String>,
) -> Result<(), StabuseError> {
    validate_assets(&asset)?;
    let assets = transform_assets_to_uppercase(&asset);

    for (key, value) in assets.clone() {
        match hashmap_to_json_value(asset.clone()) {
            Ok(_) => {
                sqlx::query(ADD_ASSET)
                    .bind(key)
                    .bind(value)
                    .bind(chain_id)
                    .bind(admin_username)
                    .execute(pool)
                    .await?;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}

pub async fn get_network_supported_assets(
    pool: &PgPool,
    chain_id: i64,
) -> Result<Asset, StabuseError> {
    let result: (Value,) = sqlx::query_as(GET_NETWORK_ASSETS)
        .bind(chain_id)
        .fetch_one(pool)
        .await?;

    let assets: Asset =
        serde_json::from_value(result.0).map_err(|e| StabuseError::SerdeError(e.to_string()))?;

    Ok(assets)
}

pub async fn get_network(pool: &PgPool, chain_id: i64) -> Result<NetworkDB, StabuseError> {
    let row = sqlx::query(GET_NETWORK)
        .bind(chain_id)
        .fetch_one(pool)
        .await?;

    let supported_assets: HashMap<String, String> =
        match row.try_get::<Value, _>("supported_assets")? {
            Value::Object(map) => map
                .into_iter()
                .map(|(k, v)| match v {
                    Value::String(s) => Ok((k, s)),
                    _ => Err("Expected string values for supported_assets".to_string()),
                })
                .collect::<Result<_, _>>()
                .expect("Error"),
            _ => {
                return Err(StabuseError::SerdeError(
                    "Invalid JSON format for supported_assets".into(),
                ))
            }
        };

    let network = NetworkDB {
        id: row.try_get("id")?,
        chain_id: row.try_get("chain_id")?,
        name: row.try_get("name")?,
        rpc_url: row.try_get("rpc")?,
        supported_assets,
        created_at: row.try_get("created_at").ok(),
        updated_at: row.try_get("updated_at").ok(),
    };

    Ok(network)
}

pub async fn get_all_networks(pool: &PgPool) -> Result<Vec<NetworkDB>, StabuseError> {
    let rows = sqlx::query(GET_ALL_NETWORKS).fetch_all(pool).await?;

    let mut networks = vec![];

    for row in rows.iter() {
        let supported_assets: HashMap<String, String> =
            match row.try_get::<Value, _>("supported_assets")? {
                Value::Object(map) => map
                    .into_iter()
                    .map(|(k, v)| match v {
                        Value::String(s) => Ok((k, s)),
                        _ => Err("Expected string values for supported_assets".to_string()),
                    })
                    .collect::<Result<_, _>>()
                    .expect("Error"),
                _ => {
                    return Err(StabuseError::SerdeError(
                        "Invalid JSON format for supported_assets".into(),
                    ))
                }
            };

        let network = NetworkDB {
            id: row.try_get("id")?,
            chain_id: row.try_get("chain_id")?,
            name: row.try_get("name")?,
            rpc_url: row.try_get("rpc")?,
            supported_assets,
            created_at: row.try_get("created_at").ok(),
            updated_at: row.try_get("updated_at").ok(),
        };

        networks.push(network);
    }

    Ok(networks)
}

pub async fn is_asset_supported_on_network(
    pool: &PgPool,
    chain_id: i64,
    asset: &str,
) -> Result<bool, StabuseError> {
    let exists: (bool,) = sqlx::query_as(CHECK_NETWORK_SUPPORTED_ASSET)
        .bind(chain_id)
        .bind(asset)
        .fetch_one(pool)
        .await?;

    Ok(exists.0)
}
