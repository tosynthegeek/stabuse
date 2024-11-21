use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use tracing::error as TracingError;

use crate::{
    error::StabuseError,
    merchant::merchant::{
        add_merchant_supported_network, add_new_merchant_network_asset, create_merchant_account,
        merchant_login, remove_merchant_network_asset, update_merchant_network_address,
    },
    types::types::{
        Claims, CreateMerchantRequest, LoginCredentials, MerchantAddressRequest,
        MerchantAssetRequest, MerchantNetworkRequest,
    },
};

pub async fn create_merchant_account_handler(
    pool: web::Data<PgPool>,
    form: web::Json<CreateMerchantRequest>,
) -> impl Responder {
    let CreateMerchantRequest {
        username,
        email,
        password,
        supported_assets,
    } = form.into_inner();

    match create_merchant_account(&pool, &username, &email, &password, supported_assets).await {
        Ok(merchant_id) => HttpResponse::Created().json(serde_json::json!({
            "status": "success",
            "message": "Merchant account created successfully",
            "merchant_id": merchant_id,
        })),
        Err(err) => {
            TracingError!(error = ?err, "Error creating merchant account");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to create merchant account: {}", err),
            }))
        }
    }
}

pub async fn add_merchant_asset_handler(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    form: web::Json<MerchantAssetRequest>,
) -> impl Responder {
    let MerchantAssetRequest { chain_id, asset } = form.into_inner();
    let claims = req
        .extensions()
        .get::<Claims>()
        .expect("Claims must be present in request")
        .clone();

    let id = claims.sub;

    match add_new_merchant_network_asset(&pool, id, chain_id, &asset).await {
        Ok(networks) => HttpResponse::Created().json(serde_json::json!({
            "status": "success",
            "message": "Merchant asset added successfully",
            "networks": networks,
        })),
        Err(e) => {
            TracingError!(error = ?e, "Error adding merchant asset");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to add merchant asset: {}", e),
            }))
        }
    }
}

pub async fn remove_merchant_asset_handler(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    form: web::Json<MerchantAssetRequest>,
) -> impl Responder {
    let MerchantAssetRequest { chain_id, asset } = form.into_inner();
    let claims = req
        .extensions()
        .get::<Claims>()
        .expect("Claims must be present in request")
        .clone();

    let id = claims.sub;

    match remove_merchant_network_asset(&pool, id, chain_id, &asset).await {
        Ok(networks) => HttpResponse::Created().json(serde_json::json!({
            "status": "success",
            "message": "Merchant asset removed successfully",
            "networks": networks,
        })),
        Err(e) => {
            TracingError!(error = ?e, "Error removing merchant asset");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to remove merchant asset: {}", e),
            }))
        }
    }
}

pub async fn add_merchant_network_handler(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    form: web::Json<MerchantNetworkRequest>,
) -> impl Responder {
    let MerchantNetworkRequest {
        chain_id,
        supported_assets,
        address,
    } = form.into_inner();

    let claims = req
        .extensions()
        .get::<Claims>()
        .expect("Claims must be present in request")
        .clone();

    let id = claims.sub;

    match add_merchant_supported_network(&pool, id, chain_id, supported_assets, &address).await {
        Ok(networks) => HttpResponse::Created().json(serde_json::json!({
            "status": "success",
            "message": "Merchant networks updated successfully",
            "networks": networks,
        })),
        Err(e) => {
            TracingError!(error = ?e, "Error updating merchant networks");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to update merchant networks: {}", e),
            }))
        }
    }
}

pub async fn update_merchant_network_address_handler(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    form: web::Json<MerchantAddressRequest>,
) -> impl Responder {
    let MerchantAddressRequest { chain_id, address } = form.into_inner();
    let claims = req
        .extensions()
        .get::<Claims>()
        .expect("Claims must be present in request")
        .clone();
    let id = claims.sub;

    match update_merchant_network_address(&pool, id, chain_id, &address).await {
        Ok(networks) => HttpResponse::Created().json(serde_json::json!({
            "status": "success",
            "message": "Merchant network address updated successfully",
            "networks": networks,
        })),
        Err(e) => {
            TracingError!(error = ?e, "Error updating merchant network address");
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to update merchant network address: {}", e),
            }))
        }
    }
}

pub async fn merchant_login_handler(
    pool: web::Data<PgPool>,
    credentials: web::Json<LoginCredentials>,
) -> Result<HttpResponse, StabuseError> {
    match merchant_login(&pool, &credentials.username_or_email, &credentials.password).await {
        Ok(login_response) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": "Login Successful",
            "response": login_response,
        }))),
        Err(e) => {
            TracingError!(error = ?e, "Login error");
            Ok(HttpResponse::Unauthorized().json(json!({
                "error": "Invalid credentials"
            })))
        }
    }
}
