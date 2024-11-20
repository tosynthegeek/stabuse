use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Network {
    pub chain_id: i64,
    pub name: String,
    pub explorer: String,
    pub rpc: String,
    pub supported_assets: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NetworkDB {
    pub id: i32,
    pub chain_id: i64,
    pub name: String,
    pub rpc_url: String,
    pub supported_assets: HashMap<String, String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Merchant {
    pub username: String,
    pub email: String,
    pub password: String,
    pub supported_networks: HashMap<i32, String>, // Map the network chain ID to the client address
}

pub type Asset = HashMap<String, String>;

#[derive(Deserialize)]
pub struct AddAssetRequest {
    pub chain_id: i32,
    pub assets: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct CreateMerchantRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub supported_assets: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct CreateAdminRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct MerchantAssetRequest {
    pub chain_id: i64,
    pub asset: String,
}

#[derive(Deserialize)]
pub struct MerchantNetworkRequest {
    pub chain_id: i64,
    pub supported_assets: Vec<String>,
    pub address: String,
}

#[derive(Deserialize)]
pub struct MerchantAddressRequest {
    pub chain_id: i64,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub merchant_id: i32,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i32, // merchant ID
    pub username: String,
    pub exp: i64, // expiration timestamp
    pub iat: i64, // issued at timestamp
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminClaims {
    pub sub: String,
    pub username: String,
    pub exp: i64, // expiration timestamp
    pub iat: i64, // issued at timestamp
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MerchantCredentials {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AdminCredentials {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct LoginCredentials {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AdminInvite {
    pub email: String,
    pub token: String,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct AdminDetails {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct OTP {
    pub otp_hash: String,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyOtpRequest {
    pub otp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminInviteRequest {
    pub email: String,
}
