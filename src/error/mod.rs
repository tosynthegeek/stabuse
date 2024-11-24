use actix_web::{error::HttpError, HttpResponse, ResponseError};
use alloy::{
    dyn_abi::parser,
    transports::{RpcError, TransportErrorKind},
};
use lettre::{address::AddressError, transport::smtp::Error as SmtpTransportError};
use std::{
    env::VarError,
    fmt::{self},
    string::ParseError,
};

#[derive(Debug)]
pub enum StabuseError {
    InvalidCredentials(String),
    InvalidData(String),
    DatabaseError(sqlx::Error),
    SerdeError(String),
    HashError(bcrypt::BcryptError),
    StdError(Box<dyn std::error::Error + Send + Sync>),
    InvalidAssetFormat(String),
    AssetNotSupportedonNetwork(String),
    JWTError(String),
    HttpError(HttpError),
    Forbidden(String),
    Unauthorized(String),
    Internal(String),
    EmailError(String),
    SmtpError(String),
    EnvError(String),
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
            StabuseError::Forbidden(msg) => write!(f, "Fobidden: {}", msg),
            StabuseError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            StabuseError::Internal(msg) => write!(f, "Internal: {}", msg),
            StabuseError::EmailError(msg) => write!(f, "Error sending mail: {}", msg),
            StabuseError::SmtpError(msg) => write!(f, "Error sending mail: {}", msg),
            StabuseError::EnvError(msg) => write!(f, "Error reading from env: {}", msg),
            StabuseError::StdError(e) => write!(f, "{}", e),
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

impl From<AddressError> for StabuseError {
    fn from(error: AddressError) -> Self {
        StabuseError::EmailError(error.to_string())
    }
}

impl From<SmtpTransportError> for StabuseError {
    fn from(error: SmtpTransportError) -> Self {
        StabuseError::SmtpError(error.to_string())
    }
}

impl From<VarError> for StabuseError {
    fn from(error: VarError) -> Self {
        StabuseError::EnvError(error.to_string())
    }
}

impl From<RpcError<TransportErrorKind>> for StabuseError {
    fn from(err: RpcError<TransportErrorKind>) -> Self {
        StabuseError::Internal(err.to_string())
    }
}

impl From<parser::Error> for StabuseError {
    fn from(error: parser::Error) -> Self {
        StabuseError::Internal(error.to_string())
    }
}

impl From<std::io::Error> for StabuseError {
    fn from(error: std::io::Error) -> Self {
        StabuseError::StdError(Box::new(error))
    }
}

impl From<serde_json::Error> for StabuseError {
    fn from(error: serde_json::Error) -> Self {
        StabuseError::SerdeError(error.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for StabuseError {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        StabuseError::EnvError(error.to_string())
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
            StabuseError::Forbidden(msg) => {
                HttpResponse::Forbidden().json(serde_json::json!({"error": msg}))
            }
            StabuseError::Unauthorized(msg) => {
                HttpResponse::Unauthorized().json(serde_json::json!({"error": msg}))
            }
            StabuseError::StdError(msg) => HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": msg.to_string()})),
            StabuseError::EmailError(msg) => HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": msg.to_string()})),
            StabuseError::SmtpError(msg) => HttpResponse::InternalServerError()
                .json(serde_json::json!({"error": msg.to_string()})),
            StabuseError::EnvError(msg) => {
                HttpResponse::NotFound().json(serde_json::json!({"error": msg.to_string()}))
            }
            StabuseError::Internal(msg) => {
                HttpResponse::NotFound().json(serde_json::json!({"error": msg.to_string()}))
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
            StabuseError::InvalidCredentials(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            StabuseError::Forbidden(_) => actix_web::http::StatusCode::FORBIDDEN,
            StabuseError::Unauthorized(_) => actix_web::http::StatusCode::UNAUTHORIZED,
            StabuseError::StdError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            StabuseError::EmailError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            StabuseError::SmtpError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            StabuseError::EnvError(_) => actix_web::http::StatusCode::NOT_FOUND,
            StabuseError::Internal(_) => actix_web::http::StatusCode::NOT_FOUND,
        }
    }
}
