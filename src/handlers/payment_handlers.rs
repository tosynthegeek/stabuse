use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde_json::json;
use sqlx::PgPool;
use tracing::error as TracingError;

use crate::{
    core::{evm::evm::create_payment_request, sol::sol::create_payment_transaction},
    db::migrations::payments::select_queries::GET_PAYMENT_EXISTENCE_BY_HASH,
    error::StabuseError,
    mq::mq::publish_message,
    types::types::{
        CreatePaymentRequest, PaymentClaims, TransactionVerificationMessage, ValidatePaymentRequest,
    },
};

pub async fn create_payment_request_handler(
    pool: web::Data<PgPool>,
    body: web::Json<CreatePaymentRequest>,
) -> Result<HttpResponse, StabuseError> {
    let data = body.into_inner();
    if data.network.to_lowercase().contains("sol") {
        match create_payment_transaction(
            &pool,
            &data.rpc_url,
            &data.user_address,
            data.merchant_id,
            &data.asset,
            data.payment_amount,
        )
        .await
        {
            Ok((tx, token)) => Ok(HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "message": "Payment creation Successful",
                "transaction": tx,
                "token": token.jwt_token,
                "webhook_url": token.webhook_url
            }))),
            Err(e) => {
                TracingError!(error = ?e, "Payment creation error");
                Ok(HttpResponse::Unauthorized().json(json!({
                    "error": "Invalid credentials"
                })))
            }
        }
    } else {
        match create_payment_request(
            &pool,
            data.merchant_id,
            data.payment_amount,
            &data.user_address,
            &data.rpc_url,
            &data.asset,
        )
        .await
        {
            Ok((tx, token)) => Ok(HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "message": "Payment creation Successful",
                "transaction": tx,
                "token": token.jwt_token,
                "webhook_url": token.webhook_url
            }))),
            Err(e) => {
                TracingError!(error = ?e, "Payment creation error");
                Ok(HttpResponse::Unauthorized().json(json!({
                    "error": "Invalid credentials"
                })))
            }
        }
    }
}

pub async fn validate_payment_handler(
    req: HttpRequest,
    body: web::Json<ValidatePaymentRequest>,
) -> Result<HttpResponse, StabuseError> {
    let data = body.into_inner();
    let claims = req
        .extensions()
        .get::<PaymentClaims>()
        .expect("Claims must be present in request")
        .clone();

    let message = TransactionVerificationMessage {
        pending_payment_id: claims.pending_payment_id,
        tx_hash: data.tx_hash,
        rpc_url: data.rpc_url,
        network: claims.network,
    };

    let rabbitmq_url =
        std::env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set in environment variables");
    let queue_name =
        std::env::var("QUEUE_NAME").expect("QUEUE_NAME must be set in environment variables");

    match publish_message(&rabbitmq_url, &queue_name, message).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": "Payment validation request sent successfully"
        }))),
        Err(e) => {
            tracing::error!(error = ?e, "Failed to publish payment validation message");
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to process payment validation request"
            })))
        }
    }
}

pub async fn confirm_payment_transaction(
    pool: web::Data<PgPool>,
    tx_hash: web::Path<String>,
) -> Result<HttpResponse, StabuseError> {
    let exists: bool = sqlx::query_scalar(GET_PAYMENT_EXISTENCE_BY_HASH)
        .bind(&tx_hash.into_inner())
        .fetch_one(pool.get_ref())
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    if exists {
        Ok(HttpResponse::Ok().json(json!({ "status": "confirmed" })))
    } else {
        Ok(HttpResponse::Ok().json(json!({ "status": "not_found" })))
    }
}
