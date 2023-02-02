use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

pub struct AppError {
    pub message: String,
    pub code: StatusCode,
}

impl AppError {
    pub fn new(message: impl Into<String>, code: StatusCode) -> Self {
        Self {
            message: message.into(),
            code,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.code,
            Json(ErrorResponse {
                error_message: self.message.clone(),
            }),
        )
            .into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    error_message: String,
}
