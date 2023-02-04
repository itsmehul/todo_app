use axum::http::StatusCode;
use bcrypt::verify;

use crate::utilities::app_error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|error| {
        eprintln!("Error hashing password: {:?}", error);
        AppError::new(
            "Error hashing password".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(|error| {
        eprintln!("Error verifying password: {:?}", error);
        AppError::new(
            "The was a problem verifying your password",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })
}
