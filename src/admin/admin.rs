use std::fmt::format;

use actix_web::{web, HttpResponse};
use chrono::{Duration, NaiveDateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::{
    db::migrations::admins::{
        insert_and_updates::{ADD_ADMIN, ADD_ADMIN_INVITE, ADD_SUPER_ADMIN, DELETE_ADMIN_INVITE},
        select_queries::{GET_ADMIN_COUNT, GET_INVITE_DETAILS},
    },
    error::StabuseError,
    types::types::{AdminDetails, AdminInvite},
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
