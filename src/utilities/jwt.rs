use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    username: String,
}

pub fn create_token(secret: &str, username: String) -> Result<String, AppError> {
    // add at least an hour for this timestamp
    let now = chrono::Utc::now();
    let expires_at = Duration::hours(1);
    let expires_at = now + expires_at;
    let exp = expires_at.timestamp() as usize;
    let claims = Claims { exp, username };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());

    encode(&token_header, &claims, &key).map_err(|error| {
        eprintln!("Error creating token: {:?}", error);
        AppError::new(
            "There was an error, please try again later",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })
}

pub fn validate_token(secret: &str, token: &str) -> Result<bool, AppError> {
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    decode::<Claims>(token, &key, &validation)
        .map_err(|error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::new("Not authenticated!", StatusCode::UNAUTHORIZED)
            }
            _ => {
                eprintln!("Error validating token: {:?}", error);
                AppError::new("Error validating token", StatusCode::INTERNAL_SERVER_ERROR)
            }
        })
        .map(|_claim| true)
}
