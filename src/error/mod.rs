use actix_web::{error::HttpError, HttpResponse, ResponseError};
use std::fmt::{self};

#[derive(Debug)]
pub enum StabuseError {
    InvalidCredentials(String),
    InvalidData(String),
    DatabaseError(sqlx::Error),
    SerdeError(String),
    HashError(bcrypt::BcryptError),
    InvalidAssetFormat(String),
    AssetNotSupportedonNetwork(String),
    JWTError(String),
    HttpError(HttpError),
}

impl fmt::Display for StabuseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StabuseError::InvalidCredentials(msg) => write!(f, "Invalid Credentials: {}", msg),
            StabuseError::InvalidData(msg) => write!(f, "Invalid Data: {}", msg),
            StabuseError::SerdeError(err) => write!(f, "Serialization error: {}", err),
            StabuseError::DatabaseError(err) => write!(f, "Database error: {}", err),
            StabuseError::HashError(err) => write!(f, "Hashing error: {}", err),
            StabuseError::InvalidAssetFormat(msg) => write!(f, "Invalid asset: {}", msg),
            StabuseError::AssetNotSupportedonNetwork(msg) => {
                write!(f, "Asset not supported in network: {}", msg)
            }
            StabuseError::JWTError(msg) => write!(f, "JWT Error: {}", msg),
            StabuseError::HttpError(err) => write!(f, "Http Error: {}", err),
        }
    }
}

impl From<sqlx::Error> for StabuseError {
    fn from(error: sqlx::Error) -> Self {
        StabuseError::DatabaseError(error)
    }
}

impl From<bcrypt::BcryptError> for StabuseError {
    fn from(error: bcrypt::BcryptError) -> Self {
        StabuseError::HashError(error)
    }
}

impl From<HttpError> for StabuseError {
    fn from(error: HttpError) -> Self {
        StabuseError::HttpError(error)
    }
}

impl std::error::Error for StabuseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StabuseError::DatabaseError(err) => Some(err),
            StabuseError::HashError(err) => Some(err),
            StabuseError::HttpError(err) => Some(err),
            _ => None,
        }
    }
}

impl ResponseError for StabuseError {
    fn error_response(&self) -> HttpResponse {
        match self {
            StabuseError::DatabaseError(e) => HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": format!("Database error: {}", e) })),
            StabuseError::InvalidData(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({ "error": msg }))
            }
            StabuseError::SerdeError(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({ "error": msg }))
            }
            StabuseError::HashError(err) => HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": format!("Hashing error: {}", err) })),
            StabuseError::InvalidAssetFormat(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({ "error": msg }))
            }
            StabuseError::AssetNotSupportedonNetwork(msg) => {
                HttpResponse::BadRequest().json(serde_json::json!({ "error": msg }))
            }
            StabuseError::JWTError(msg) => {
                HttpResponse::Unauthorized().json(serde_json::json!({ "error": msg }))
            }
            StabuseError::HttpError(err) => HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": format!("HTTP error: {}", err) })),
            StabuseError::InvalidCredentials(msg) => {
                HttpResponse::Unauthorized().json(serde_json::json!({ "error": msg }))
            }
        }
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            StabuseError::DatabaseError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            StabuseError::InvalidData(_) => actix_web::http::StatusCode::BAD_REQUEST,
            StabuseError::SerdeError(_) => actix_web::http::StatusCode::BAD_REQUEST,
            StabuseError::HashError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            StabuseError::InvalidAssetFormat(_) => actix_web::http::StatusCode::BAD_REQUEST,
            StabuseError::AssetNotSupportedonNetwork(_) => actix_web::http::StatusCode::BAD_REQUEST,
            StabuseError::JWTError(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            StabuseError::HttpError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidCredentials(_) => actix_web::http::StatusCode::UNAUTHORIZED,
        }
    }
}
