use axum::{extract::State, http::StatusCode, Extension};
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel, Set};

use crate::{database::users, utilities::app_error::AppError};

pub async fn logout(
    Extension(user): Extension<users::Model>,
    State(db): State<DatabaseConnection>,
) -> Result<StatusCode, AppError> {
    let mut user = user.into_active_model();

    user.token = Set(None);

    user.save(&db).await.map_err(|error| {
        eprintln!("Error logging out user: {:?}", error);
        AppError::new(
            "Error logging out user".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    Ok(StatusCode::OK)
}
