use alloy::primitives::{Address, U256};
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct NetworkInfo {
    pub name: String,
    pub supported_assets: Value,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentClaims {
    pub pending_payment_id: i32,
    pub network: String,
    pub rpc: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Stablecoins {
    pub usdt: String,
    pub usdc: String,
    pub busd: String,
    pub dai: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    pub sender: String,
    pub amount: u64,
    pub tx_hash: String,
    pub asset: String,
    pub network: String,
    pub time: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePaymentRequest {
    pub merchant_id: i32,
    pub payment_amount: u64,
    pub user_address: String,
    pub asset: String,
    pub rpc_url: String,
    pub network: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidatePaymentRequest {
    pub tx_hash: String,
    pub rpc_url: String,
    pub network: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePaymentTransaction {
    pub to: String,
    pub from: String,
    pub data: String,
    pub value: String,
    pub nonce: String,
    pub chain_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fee_per_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_priority_fee_per_gas: Option<String>,
}

#[derive(Debug)]
pub struct TransactionValidationParams {
    pub merchant_address: Address,
    pub token_address: Address,
    pub user_address: Address,
    pub amount: U256,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PendingPayment {
    pub id: i32,
    pub merchant_id: i32,
    pub sender: String,
    pub amount: BigDecimal,
    pub asset: String,
    pub network: String,
    pub webhook_url: String,
    pub time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionVerificationMessage {
    pub pending_payment_id: i32,
    pub tx_hash: String,
    pub rpc_url: String,
    pub network: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentAuthDetails {
    pub jwt_token: String,
    pub webhook_url: String,
}

#[derive(Serialize)]
pub struct WebhookPayload {
    pub payment_id: i32,
    pub status: String,
    pub tx_hash: String,
    pub timestamp: String,
}
