use actix_web::{dev::ServiceRequest, HttpMessage};
use actix_web_httpauth::extractors::{bearer::{BearerAuth, Config}, AuthenticationError};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::{error::StabuseError, types::types::Claims};

pub fn generate_jwt(
    merchant_id: i32,
    username: &str,
    jwt_secret: String,
) -> Result<String, StabuseError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: merchant_id,
        username: username.to_string(),
        exp: expiration,
        iat: Utc::now().timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| StabuseError::JWTError(format!("Failed to create token: {}", e)))
}

pub async fn verify_jwt(token: &str, jwt_secret: String) -> Result<Claims, actix_web::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|token_data| token_data.claims)
    .map_err(|e| {
        let error_message = format!("Invalid token: {}", e);
        actix_web::error::ErrorUnauthorized(error_message)
    })
}

pub async fn jwt_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token = credentials.token();
    match verify_jwt(token, jwt_secret).await {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(e) => {
            eprintln!("Token validation failed: {:?}", e);

            let config = req
                .app_data::<Config>()
                .map(|data| data.clone())
                .unwrap_or_else(Default::default);

            let auth_error = AuthenticationError::from(config).into();
            Err((auth_error, req))
        }
    }
}
