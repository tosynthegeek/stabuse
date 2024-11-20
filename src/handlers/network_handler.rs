use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use tracing::error as TracingError;

use crate::{
    network::network::{
        add_asset_to_network, add_network, get_all_networks, get_network,
        get_network_supported_assets,
    },
    types::types::{AddAssetRequest, AdminClaims, Network},
};

pub async fn handle_add_network(
    req: HttpRequest,
    body: web::Json<Network>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let claims = req
        .extensions()
        .get::<AdminClaims>()
        .expect("Claims must be present in request")
        .clone();
    let username = &claims.username;
    let msg: Network = body.into_inner();

    match add_network(&pool.into_inner(), username, msg).await {
        Ok(_) => HttpResponse::Ok().body("Added network successfully"),
        Err(e) => {
            TracingError!(error = ?e,"Error adding network");
            HttpResponse::InternalServerError().json(format!("Error adding network: {}", e))
        }
    }
}

pub async fn handle_get_network_supported_assets(
    pool: web::Data<PgPool>,
    body: web::Json<i64>,
) -> impl Responder {
    let chain_id = body.into_inner();
    match get_network_supported_assets(&pool, chain_id).await {
        Ok(assets) => HttpResponse::Ok().json(json!({
            "message": "Assets fetched successfully",
            "assets": assets
        })),
        Err(err) => {
            TracingError!(error = ?err, "Error getting assets");
            HttpResponse::InternalServerError().body("Error getting assets")
        }
    }
}

pub async fn handle_add_asset(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<AddAssetRequest>,
) -> impl Responder {
    let claims = req
        .extensions()
        .get::<AdminClaims>()
        .expect("Claims must be present in request")
        .clone();
    let username = &claims.username;
    let data = body.into_inner();

    match add_asset_to_network(&pool, username, data.chain_id, data.assets).await {
        Ok(_) => HttpResponse::Ok().body("Asset added successfully"),
        Err(err) => {
            TracingError!(error = ?err, "Error adding asset");
            HttpResponse::InternalServerError().body("Error adding asset")
        }
    }
}

pub async fn handle_get_network(
    _req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<i64>,
) -> impl Responder {
    let chain_id = body.into_inner();

    match get_network(&pool, chain_id).await {
        Ok(network) => HttpResponse::Ok().json(network),
        Err(err) => {
            TracingError!(error = ?err, "Error getting network");
            HttpResponse::InternalServerError().body("Error getting network")
        }
    }
}

pub async fn handle_get_all_networks(_req: HttpRequest, pool: web::Data<PgPool>) -> impl Responder {
    match get_all_networks(&pool).await {
        Ok(networks) => HttpResponse::Ok().json(networks),
        Err(err) => {
            TracingError!(error = ?err, "Error getting available networks");
            HttpResponse::InternalServerError().body("Error getting available networks")
        }
    }
}
