use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use serde_json::json;
use sqlx::PgPool;
use tracing::error as TracingError;

use crate::{
    core::evm::evm::{create_payment_request, verify_signed_transaction},
    error::StabuseError,
    types::types::{CreateEVMPaymentRequest, PaymentClaims, ValidateEVMPaymentRequest},
};

pub async fn evm_create_payment_request_handler(
    pool: web::Data<PgPool>,
    body: web::Json<CreateEVMPaymentRequest>,
) -> Result<HttpResponse, StabuseError> {
    let data = body.into_inner();
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
            "token": token
        }))),
        Err(e) => {
            TracingError!(error = ?e, "Payment creation error");
            Ok(HttpResponse::Unauthorized().json(json!({
                "error": "Invalid credentials"
            })))
        }
    }
}

pub async fn validate_evm_payment_handler(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<ValidateEVMPaymentRequest>,
) -> Result<HttpResponse, StabuseError> {
    let data = body.into_inner();
    let claims = req
        .extensions()
        .get::<PaymentClaims>()
        .expect("Claims must be present in request")
        .clone();

    match verify_signed_transaction(
        &pool,
        claims.pending_payment_id,
        &data.rpc_url,
        &data.tx_hash,
    )
    .await
    {
        Ok(id) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "message": "Payment creation Successful",
            "Payment id": id
        }))),
        Err(e) => {
            TracingError!(error = ?e, "Payment creation error");
            Ok(HttpResponse::Unauthorized().json(json!({
                "error": "Invalid credentials"
            })))
        }
    }
}
