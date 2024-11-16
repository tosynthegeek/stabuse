use crate::{
    db::migrations::merchants::insert_and_update_merchants::{
        ADD_ASSET_MERCHANT, ADD_MERCHANT, ADD_MERCHANT_SUPPORTED_NETWORK, REMOVE_ASSET_MERCHANT,
        UPDATE_NETWORK_ADDRESS_MERCHANT,
    },
    error::StabuseError,
    network::network::is_asset_supported_on_network,
    utils::{
        utils::{hash_password, validate_supported_assets, validate_supported_networks},
        validation::{validate_address, validate_email, validate_password, validate_username},
    },
};
use serde_json::{json, Value};
use sqlx::PgPool;

pub async fn create_merchant_account(
    pool: &PgPool,
    username: &str,
    email: &str,
    password: &str,
    supported_networks: Option<serde_json::Value>,
) -> Result<i32, StabuseError> {
    validate_username(username)?;
    if !validate_email(email) {
        return Err(StabuseError::InvalidCredentials(
            "Invalid email format".to_string(),
        ));
    }

    validate_password(password)?;

    let password_hash = hash_password(password)?;

    if let Some(ref assets) = supported_networks {
        validate_supported_networks(pool, assets).await?;
    }

    let assets = supported_networks.unwrap_or_else(|| json!({}));

    let id = sqlx::query_scalar(ADD_MERCHANT)
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(assets)
        .fetch_one(pool)
        .await?;

    Ok(id)
}

pub async fn add_new_merchant_network_asset(
    pool: &PgPool,
    username: &str,
    chain_id: i64,
    asset_value: &str,
) -> Result<Value, StabuseError> {
    let asset = asset_value.to_uppercase();
    if !is_asset_supported_on_network(pool, chain_id, &asset).await? {
        return Err(StabuseError::AssetNotSupportedonNetwork(format!(
            "Network does support {}",
            asset
        )));
    }
    let updated_networks: Value = sqlx::query_scalar(ADD_ASSET_MERCHANT)
        .bind(chain_id)
        .bind(asset)
        .bind(username)
        .fetch_one(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    Ok(updated_networks)
}

pub async fn remove_merchant_network_asset(
    pool: &PgPool,
    username: &str,
    chain_id: i64,
    asset_value: &str,
) -> Result<Value, StabuseError> {
    let asset = asset_value.to_uppercase();
    let updated_networks: Value = sqlx::query_scalar(REMOVE_ASSET_MERCHANT)
        .bind(chain_id)
        .bind(asset)
        .bind(username)
        .fetch_one(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    Ok(updated_networks)
}

pub async fn add_merchant_supported_network(
    pool: &PgPool,
    username: &str,
    chain_id: i64,
    supported_assets: Vec<String>,
    address: &str,
) -> Result<Value, StabuseError> {
    validate_supported_assets(pool, chain_id, supported_assets.clone()).await?;
    validate_address(address)?;

    let networks = sqlx::query_scalar(ADD_MERCHANT_SUPPORTED_NETWORK)
        .bind(username)
        .bind(chain_id)
        .bind(supported_assets)
        .bind(address)
        .fetch_one(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    Ok(networks)
}

pub async fn update_merchant_network_address(
    pool: &PgPool,
    username: &str,
    chain_id: i64,
    address: &str,
) -> Result<Value, StabuseError> {
    validate_address(address)?;
    let updated_networks: Value = sqlx::query_scalar(UPDATE_NETWORK_ADDRESS_MERCHANT)
        .bind(chain_id)
        .bind(address)
        .bind(username)
        .fetch_one(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    Ok(updated_networks)
}
