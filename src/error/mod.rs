use std::fmt::{self};

#[derive(Debug)]
pub enum StabuseError {
    InvalidCredentials(String),
    InvalidData(String),
    DatabaseError(sqlx::Error),
    SerdeError(String),
    HashError(String),
    InvalidAssetFormat(String),
    AssetNotSupportedonNetwork(String),
}

impl fmt::Display for StabuseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StabuseError::InvalidCredentials(msg) => write!(f, "Invalid Credentials: {}", msg),
            StabuseError::InvalidData(msg) => write!(f, "Invalid Data: {}", msg),
            StabuseError::SerdeError(err) => write!(f, "Serialization error: {}", err),
            StabuseError::DatabaseError(err) => write!(f, "Database error: {}", err),
            StabuseError::HashError(err) => write!(f, "Hashing error {}", err),
            StabuseError::InvalidAssetFormat(msg) => write!(f, "Invalid asset {}", msg),
            StabuseError::AssetNotSupportedonNetwork(msg) => {
                write!(f, "Asset not Support in Network {}", msg)
            }
        }
    }
}

impl From<sqlx::Error> for StabuseError {
    fn from(error: sqlx::Error) -> Self {
        StabuseError::DatabaseError(error)
    }
}

impl std::error::Error for StabuseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StabuseError::DatabaseError(msg) => Some(msg),
            _ => None,
        }
    }
}
