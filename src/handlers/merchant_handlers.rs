use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use tracing::error as TracingError;

use crate::{
    merchant::merchant::{
        add_merchant_supported_network, add_new_merchant_network_asset, create_merchant_account,
        remove_merchant_network_asset, update_merchant_network_address,
    },
    types::types::{
        CreateMerchantRequest, MerchantAddressRequest, MerchantAssetRequest, MerchantNetworkRequest,
    },
};

pub async fn create_merchant_account_handler(
    _req: HttpRequest,
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
    _req: HttpRequest,
    pool: web::Data<PgPool>,
    form: web::Json<MerchantAssetRequest>,
) -> impl Responder {
    let MerchantAssetRequest {
        username,
        chain_id,
        asset,
    } = form.into_inner();

    match add_new_merchant_network_asset(&pool, &username, chain_id, &asset).await {
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
    _req: HttpRequest,
    pool: web::Data<PgPool>,
    form: web::Json<MerchantAssetRequest>,
) -> impl Responder {
    let MerchantAssetRequest {
        username,
        chain_id,
        asset,
    } = form.into_inner();

    match remove_merchant_network_asset(&pool, &username, chain_id, &asset).await {
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
    _req: HttpRequest,
    pool: web::Data<PgPool>,
    form: web::Json<MerchantNetworkRequest>,
) -> impl Responder {
    let MerchantNetworkRequest {
        username,
        chain_id,
        supported_assets,
        address,
    } = form.into_inner();

    match add_merchant_supported_network(&pool, &username, chain_id, supported_assets, &address)
        .await
    {
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
    _req: HttpRequest,
    pool: web::Data<PgPool>,
    form: web::Json<MerchantAddressRequest>,
) -> impl Responder {
    let MerchantAddressRequest {
        username,
        chain_id,
        address,
    } = form.into_inner();

    match update_merchant_network_address(&pool, &username, chain_id, &address).await {
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
