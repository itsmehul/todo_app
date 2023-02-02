use axum::http::StatusCode;

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
