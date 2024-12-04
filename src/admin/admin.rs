use std::env;

use bcrypt::verify;
use chrono::{Duration, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::{
    auth::{
        jwt::generate_admin_jwt,
        otp::{send_otp, verify_otp, EmailConfig},
    },
    db::migrations::admins::{
        insert_and_updates::{ADD_ADMIN, ADD_ADMIN_INVITE, ADD_SUPER_ADMIN, DELETE_ADMIN_INVITE},
        select_queries::{GET_ADMIN_COUNT, GET_INVITE_DETAILS, LOGIN_ATTEMPT},
    },
    error::StabuseError,
    types::types::{AdminCredentials, AdminInvite},
    utils::utils::hash_password,
};

pub async fn create_super_admin(
    pool: &PgPool,
    email: &str,
    username: &str,
    password: &str,
) -> Result<i32, StabuseError> {
    let admin_count: i64 = sqlx::query(GET_ADMIN_COUNT)
        .fetch_one(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?
        .get(0);

    if admin_count > 0 {
        return Err(StabuseError::Forbidden(format!(
            "Admin already exists. Use an invite to create more admins."
        )));
    }

    let hashed_password = hash_password(password)?;
    let id: i32 = sqlx::query_scalar(ADD_SUPER_ADMIN)
        .bind(email)
        .bind(username)
        .bind(&hashed_password)
        .fetch_one(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    Ok(id)
}

pub async fn generate_admin_invite(pool: &PgPool, email: String) -> Result<i32, StabuseError> {
    let token = Uuid::new_v4().to_string();
    let expiration_time = Utc::now() + Duration::minutes(10);
    let hashed_token = hash_password(&token)?;

    let id = sqlx::query_scalar(ADD_ADMIN_INVITE)
        .bind(&email)
        .bind(&hashed_token)
        .bind(expiration_time)
        .fetch_one(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    Ok(id)
}

pub async fn create_admin_with_invite(
    pool: &PgPool,
    email: &str,
    username: &str,
    password: &str,
) -> Result<i32, StabuseError> {
    let invite = sqlx::query_as::<_, AdminInvite>(GET_INVITE_DETAILS)
        .bind(email)
        .fetch_optional(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    let invite = match invite {
        Some(invite) => invite,
        None => {
            return Err(StabuseError::InvalidCredentials(format!(
                "No invite for this email"
            )))
        }
    };

    if Utc::now().naive_utc() > invite.expires_at {
        sqlx::query(DELETE_ADMIN_INVITE)
            .bind(&invite.email)
            .execute(pool)
            .await
            .map_err(|e| StabuseError::DatabaseError(e))?;
        return Err(StabuseError::Unauthorized(format!("Invite has expired")));
    }

    let hashed_password = hash_password(password)?;
    let id = sqlx::query_scalar(ADD_ADMIN)
        .bind(&invite.email)
        .bind(username)
        .bind(&hashed_password)
        .fetch_one(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    sqlx::query(DELETE_ADMIN_INVITE)
        .bind(&invite.email)
        .execute(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    Ok(id)
}

pub async fn admin_login_request(
    pool: &PgPool,
    username_or_email: &str,
    password: &str,
) -> Result<(String, String), StabuseError> {
    let admin_credentials = sqlx::query_as::<_, AdminCredentials>(LOGIN_ATTEMPT)
        .bind(username_or_email)
        .fetch_optional(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    let admin_credentials = match admin_credentials {
        Some(admin_credentials) => admin_credentials,
        None => {
            return Err(StabuseError::InvalidCredentials(format!(
                "Admin doesnt exist"
            )))
        }
    };

    if !verify(password, &admin_credentials.password_hash)? {
        return Err(StabuseError::InvalidCredentials(format!(
            "Incorrect Password"
        )));
    }

    let config = &EmailConfig::from_env()?;

    send_otp(pool, &admin_credentials.email, config).await?;

    Ok((admin_credentials.email, admin_credentials.username))
}

pub async fn verify_otp_and_login(
    pool: &PgPool,
    email: &str,
    username: &str,
    otp: &str,
) -> Result<String, StabuseError> {
    verify_otp(pool, email, otp).await?;

    dotenv::dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

    let jwt = generate_admin_jwt(email, username, &jwt_secret)?;

    Ok(jwt)
}
