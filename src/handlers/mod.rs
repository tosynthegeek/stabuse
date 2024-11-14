use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use tracing::error as TracingError;

use crate::{db::db_init::init_db, network::network::add_network, types::types::Network};

pub async fn handle_add_network(_req: HttpRequest, body: web::Json<Network>, pool: web::Data<PgPool>) -> impl Responder {
    let msg: Network = body.into_inner();

    match add_network(&pool.into_inner(), msg).await {
        Ok(_) => {
            HttpResponse::Ok().body("Added network successfully" )
        },   
        Err(e) => {
            TracingError!(error = ?e,"Error adding network");
            HttpResponse::InternalServerError().json(format!("Error adding network: {}", e))
        }
    }
}

pub async fn handle_init_bd(_req: HttpRequest, pool: web::Data<PgPool>) -> impl Responder {
    match init_db(&pool.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("Tables created successfully"),
        Err(err) => {
            TracingError!(error = ?err, "Error initializing db");
            HttpResponse::InternalServerError().body("Error initializing db")
        }
    }
}