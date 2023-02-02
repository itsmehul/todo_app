use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
}

pub fn create_token(secret: &str) -> Result<String, AppError> {
    let now = chrono::Utc::now().timestamp() as usize;
    let expires_at = Duration::hours(1);
    let exp = now + expires_at.num_seconds() as usize;

    let claims = Claims { exp };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret.as_bytes());
    encode(&token_header, &claims, &key).map_err(|error| {
        eprintln!("Error creating token: {:?}", error);
        AppError::new(
            "Error creating token".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })
}