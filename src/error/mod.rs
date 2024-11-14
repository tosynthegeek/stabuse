use std::fmt::{self};
use serde_json::Error as SerdeError;

#[derive(Debug)]
pub enum StabuseError {
    DatabaseError(sqlx::Error),
    SerdeError(SerdeError),
    InvalidAssetFormat(String)
}

impl fmt::Display for StabuseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StabuseError::SerdeError(err) => write!(f, "Serialization error: {}", err),
            StabuseError::DatabaseError(err) => write!(f, "Database error: {}", err),
            StabuseError::InvalidAssetFormat(msg ) => write!(f, "Invalid asset {}", msg)
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
            _ => None
        }
    }
}