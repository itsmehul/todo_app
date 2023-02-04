use crate::{
    database::{
        users::Model as UserModel,
        users::{self, Entity as Users},
    },
    utilities::app_error::AppError,
};
use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TryIntoModel,
};

pub async fn save_active_user(
    db: &DatabaseConnection,
    user: users::ActiveModel,
) -> Result<UserModel, AppError> {
    let user = user.save(db).await.map_err(|error| {
        let error_message = error.to_string();

        if error_message
            .contains("duplicate key value violates unique constraint \"users_username_key\"")
        {
            AppError::new(
                "Username already taken, try again with a different user name",
                StatusCode::BAD_REQUEST,
            )
        } else {
            eprintln!("Error creating user: {:?}", error_message);
            AppError::new(
                "Something went wrong, please try again",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    })?;

    convert_active_to_model(user)
}

pub async fn find_by_username(
    db: &DatabaseConnection,
    username: String,
) -> Result<UserModel, AppError> {
    Users::find()
        .filter(users::Column::Username.eq(username))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting user by username: {:?}", error);
            AppError::new(
                "Error logging in, please try again later",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?
        .ok_or_else(|| {
            AppError::new(
                "incorrect username and/or password",
                StatusCode::BAD_REQUEST,
            )
        })
}

fn convert_active_to_model(active_user: users::ActiveModel) -> Result<UserModel, AppError> {
    active_user.try_into_model().map_err(|error| {
        eprintln!("Error converting task active model to model: {:?}", error);
        AppError::new("Internal server error", StatusCode::INTERNAL_SERVER_ERROR)
    })
}
