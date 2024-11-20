use std::env;

use crate::{
    auth::jwt::generate_merchant_jwt,
    db::migrations::merchants::{
        insert_and_update_merchants::{
            ADD_ASSET_MERCHANT, ADD_MERCHANT, ADD_MERCHANT_SUPPORTED_NETWORK,
            REMOVE_ASSET_MERCHANT, UPDATE_NETWORK_ADDRESS_MERCHANT,
        },
        select_queries::LOGIN_ATTEMPT,
    },
    error::StabuseError,
    network::network::is_asset_supported_on_network,
    types::types::{LoginResponse, MerchantCredentials},
    utils::{
        utils::hash_password,
        validation::{
            address_validation::validate_address,
            domain_validation::{validate_supported_assets, validate_supported_networks},
            input_validation::{validate_email, validate_password, validate_username},
        },
    },
};
use bcrypt::verify;
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

pub async fn merchant_login(
    pool: &PgPool,
    username_or_email: &str,
    password: &str,
) -> Result<LoginResponse, StabuseError> {
    let merchant = sqlx::query_as::<_, MerchantCredentials>(LOGIN_ATTEMPT)
        .bind(username_or_email)
        .fetch_optional(pool)
        .await?;

    let merchant = match merchant {
        Some(merchant) => merchant,
        None => {
            return Err(StabuseError::InvalidCredentials(format!(
                "Incorrect Password"
            )))
        }
    };

    if !verify(password, &merchant.password_hash)? {
        return Err(StabuseError::InvalidCredentials(
            "Invalid credentials".to_string(),
        ));
    }

    dotenv::dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

    let token = generate_merchant_jwt(merchant.id, &merchant.username, jwt_secret)?;

    Ok(LoginResponse {
        token,
        merchant_id: merchant.id,
        username: merchant.username,
    })
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
        .bind(chain_id.to_string())
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
        .bind(chain_id.to_string())
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
