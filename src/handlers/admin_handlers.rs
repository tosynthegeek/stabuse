use actix_web::{cookie::Cookie, web, HttpRequest, HttpResponse};
use serde_json::json;
use sqlx::PgPool;
use tracing::error as TracingError;

use crate::{
    admin::admin::{
        admin_login_request, create_admin_with_invite, create_super_admin, generate_admin_invite,
        verify_otp_and_login,
    },
    error::StabuseError,
    types::types::{
        AdminDetails, AdminInviteRequest, CreateAdminRequest, LoginCredentials, VerifyOtpRequest,
    },
};

pub async fn admin_login_handler(
    pool: web::Data<PgPool>,
    form: web::Json<LoginCredentials>,
) -> Result<HttpResponse, StabuseError> {
    let (email, username) =
        admin_login_request(&pool, &form.username_or_email, &form.password).await?;

    let is_secure = std::env::var("IS_SECURE").unwrap_or("false".to_string()) == "true";

    let response = HttpResponse::Ok()
        .append_header(("Location", "/admin/auth/otp/verify"))
        .cookie(
            Cookie::build("admin_email", email)
                .secure(is_secure) // Set based on environment
                .http_only(true)
                .finish(),
        )
        .cookie(
            Cookie::build("admin_username", username)
                .secure(is_secure) // Set based on environment
                .http_only(true)
                .finish(),
        )
        .json("OTP sent to email");

    Ok(response)
}

pub async fn verify_otp_handler(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    form: web::Json<VerifyOtpRequest>,
) -> Result<HttpResponse, StabuseError> {
    let email = req
        .cookie("admin_email")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| StabuseError::InvalidData("Missing admin email cookie".to_string()))?;

    let username = req
        .cookie("admin_username")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| StabuseError::InvalidData("Missing admin username cookie".to_string()))?;

    let jwt = verify_otp_and_login(&pool, &email, &username, &form.otp).await?;

    Ok(HttpResponse::Ok().json(json!({ "token": jwt })))
}

pub async fn create_super_admin_handler(
    pool: web::Data<PgPool>,
    form: web::Json<CreateAdminRequest>,
) -> Result<HttpResponse, StabuseError> {
    let data = form.into_inner();

    match create_super_admin(&pool, &data.email, &data.username, &data.password).await {
        Ok(admin_id) => Ok(HttpResponse::Created().json(serde_json::json!({
            "status": "success",
            "message": "Super Admin created successfully",
            "admin_id": admin_id,
        }))),
        Err(err) => {
            TracingError!(error = ?err, "Error creating super admin account");
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to create super admin account: {}", err),
            })))
        }
    }
}

pub async fn generate_admin_invite_handler(
    pool: web::Data<PgPool>,
    form: web::Json<AdminInviteRequest>,
) -> Result<HttpResponse, StabuseError> {
    let data = form.into_inner();

    match generate_admin_invite(&pool, data.email).await {
        Ok(invite_id) => Ok(HttpResponse::Created().json(serde_json::json!({
            "status": "success",
            "message": "Admin invite created successfully",
            "invite_id": invite_id,
        }))),
        Err(err) => {
            TracingError!(error = ?err, "Error creating admin invite");
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to create admin invite: {}", err),
            })))
        }
    }
}

pub async fn create_admin_with_invite_handler(
    pool: web::Data<PgPool>,
    form: web::Json<AdminDetails>,
) -> Result<HttpResponse, StabuseError> {
    let data = form.into_inner();

    match create_admin_with_invite(&pool, &data.email, &data.username, &data.password).await {
        Ok(admin_id) => Ok(HttpResponse::Created().json(serde_json::json!({
            "status": "success",
            "message": "Admin account created successfully",
            "admin_id": admin_id,
        }))),
        Err(err) => {
            TracingError!(error = ?err, "Error creating admin account");
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("Failed to create admin account: {}", err),
            })))
        }
    }
}
