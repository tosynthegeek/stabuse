pub mod admin_handlers;
pub mod merchant_handlers;
pub mod network_handler;
pub mod payment_handlers;

use crate::db::db_init::init_db;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sqlx::PgPool;
use tracing::error as TracingError;

pub async fn handle_init_bd(_req: HttpRequest, pool: web::Data<PgPool>) -> impl Responder {
    match init_db(&pool.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("DB initialized successfully"),
        Err(err) => {
            TracingError!(error = ?err, "Error initializing db");
            HttpResponse::InternalServerError().body("Error initializing db")
        }
    }
}
