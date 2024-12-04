use std::env;

use crate::db::migrations::admins::insert_and_updates::DELETE_OTP;
use crate::db::migrations::admins::select_queries::GET_OTP;
use crate::types::types::OTP;
use crate::utils::utils::hash_password;
use crate::{db::migrations::admins::insert_and_updates::ADD_OTP, error::StabuseError};
use bcrypt::verify;
use chrono::{Duration, Utc};
use dotenv::dotenv;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, SmtpTransport, Transport};
use rand::Rng;
use sqlx::{PgPool, Row};

pub struct EmailConfig {
    smtp_username: String,
    smtp_password: String,
    smtp_server: String,
    smtp_port: u16,
    sender_email: String,
}

impl EmailConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();

        Ok(EmailConfig {
            smtp_username: env::var("SMTP_USERNAME")?,
            smtp_password: env::var("SMTP_PASSWORD")?,
            smtp_server: env::var("SMTP_SERVER")?,
            smtp_port: env::var("SMTP_PORT")?.parse().unwrap_or(587),
            sender_email: env::var("SENDER_EMAIL")?,
        })
    }
}

fn generate_otp() -> String {
    let mut rng = rand::thread_rng();
    let otp: u32 = rng.gen_range(100000..999999);
    otp.to_string()
}

pub async fn send_otp(
    pool: &PgPool,
    admin_email: &str,
    config: &EmailConfig,
) -> Result<(), StabuseError> {
    let otp = generate_otp();
    let otp_hash = hash_password(&otp)?;
    let expires_at = Utc::now() + Duration::minutes(10);
    let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

    let _otp_id: i32 = sqlx::query(ADD_OTP)
        .bind(admin_email)
        .bind(otp_hash)
        .bind(expires_at)
        .fetch_one(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?
        .get(0);

    let tls_parameters = TlsParameters::builder(config.smtp_server.clone())
        .build()
        .map_err(|e| StabuseError::SmtpError(format!("TLS configuration error: {}", e)))?;

    let email = Message::builder()
        .from(config.sender_email.parse()?)
        .to(admin_email.parse()?)
        .subject("Your Admin Login OTP")
        .body(format!(
            "Your OTP is: {}\nThis OTP is valid for 10 minutes.",
            otp
        ))
        .map_err(|e| StabuseError::EmailError(format!("Failed to construct email: {}", e)))?;

    let mailer = SmtpTransport::relay(&config.smtp_server)?
        .port(config.smtp_port)
        .credentials(creds)
        .tls(Tls::Required(tls_parameters))
        .build();

    mailer
        .send(&email)
        .map_err(|e| StabuseError::SmtpError(format!("Failed to send OTP email: {}", e)))?;

    Ok(())
}

pub async fn verify_otp(pool: &PgPool, email: &str, otp: &str) -> Result<(), StabuseError> {
    let otp_details = sqlx::query_as::<_, OTP>(GET_OTP)
        .bind(email)
        .fetch_optional(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    let otp_details = match otp_details {
        Some(otp_details) => otp_details,
        None => {
            return Err(StabuseError::InvalidData(
                "OTP record not found".to_string(),
            ))
        }
    };

    if Utc::now().naive_utc() > otp_details.expires_at {
        sqlx::query(DELETE_OTP)
            .bind(email)
            .execute(pool)
            .await
            .map_err(|e| StabuseError::DatabaseError(e))?;
        return Err(StabuseError::InvalidData("OTP has expired".to_string()));
    }

    if !verify(otp, &otp_details.otp_hash)? {
        return Err(StabuseError::InvalidData("Invalid OTP".to_string()));
    }

    sqlx::query(DELETE_OTP)
        .bind(email)
        .execute(pool)
        .await
        .map_err(|e| StabuseError::DatabaseError(e))?;

    Ok(())
}
