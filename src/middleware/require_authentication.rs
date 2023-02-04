use crate::database::users::{self, Entity as Users};
use axum::{
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::utilities::{app_error::AppError, jwt::validate_token, token_wrapper::TokenWrapper};

pub async fn require_authentication<T>(
    header: HeaderMap,
    State(db): State<DatabaseConnection>,
    State(token_secret): State<TokenWrapper>,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    let header_token = if let Some(token) = header.get("auth-token") {
        token.to_str().map_err(|error| {
            eprintln!("Error parsing token: {:?}", error);
            AppError::new(
                "Error parsing token".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?
    } else {
        return Err(AppError::new(
            "Missing auth token",
            StatusCode::UNAUTHORIZED,
        ));
    };

    validate_token(&token_secret.0, &header_token)?;

    let user = Users::find()
        .filter(users::Column::Token.eq(header_token.to_owned()))
        .one(&db)
        .await
        .map_err(|error| {
            eprintln!("Error getting user by token: {:?}", error);
            AppError::new(
                "Error logging in, please try again later",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    if let Some(user) = user {
        request.extensions_mut().insert(user);
    } else {
        return Err(AppError::new(
            "Missing auth token",
            StatusCode::UNAUTHORIZED,
        ));
    }
    Ok(next.run(request).await)
}
