use axum::{extract::State, http::StatusCode, Json};
use jsonwebtoken::{decode, DecodingKey, TokenData};
use sea_orm::{ActiveModelTrait, TryIntoModel};

use crate::{
    database::users,
    utilities::{
        app_error::AppError, hash::hash_password, jwt::create_token, token_wrapper::TokenWrapper,
    },
};
use sea_orm::{DatabaseConnection, Set};

use super::{RequestCreateUser, ResponseDataUser, ResponseUser};

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    State(jwt_token): State<TokenWrapper>,
    Json(request_user): Json<RequestCreateUser>,
) -> Result<Json<ResponseDataUser>, AppError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };
    new_user.username = Set(request_user.username.clone());
    new_user.password = Set(hash_password(&request_user.password)?);
    new_user.token = Set(Some(create_token(&jwt_token.0, request_user.username)?));

    let user = new_user
        .save(&db)
        .await
        .map_err(|error| {
            let error_message = error.to_string();
            if error_message.contains("duplicate key value violates unique constraint") {
                return AppError::new(
                    "Username already exists".to_string(),
                    StatusCode::BAD_REQUEST,
                );
            }
            eprintln!("Error creating user: {:?}", error_message);
            AppError::new(
                "Error creating user".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?
        .try_into_model()
        .map_err(|error| {
            eprintln!("Error creating user: {:?}", error);
            AppError::new(
                "Error creating user".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let response_user = ResponseUser {
        id: user.id,
        username: user.username,
        token: user.token.unwrap(),
    };
    let response_data_user = ResponseDataUser {
        data: response_user,
    };
    Ok(Json(response_data_user))
}
