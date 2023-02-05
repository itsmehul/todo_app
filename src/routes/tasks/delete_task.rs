use crate::{
    database::{
        tasks::{self, Entity as Task},
        users::Model as UserModel,
    },
    utilities::app_error::AppError,
};
use axum::{
    extract::{Path, State},
    Extension,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

pub async fn soft_delete_task(
    Extension(user): Extension<UserModel>,
    Path(task_id): Path<i32>,
    State(db): State<DatabaseConnection>,
) -> Result<(), AppError> {
    let mut task = Task::find()
        .filter(
            tasks::Column::Id
                .eq(task_id)
                .and(tasks::Column::UserId.eq(user.id)),
        )
        .one(&db)
        .await
        .map_err(|_| {
            AppError::new(
                "Error getting task",
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?
        .ok_or_else(|| AppError::new("Task not found", axum::http::StatusCode::NOT_FOUND))?
        .into_active_model();

    task.deleted_at = Set(Some(chrono::Utc::now().into()));

    task.save(&db).await.map_err(|_| {
        AppError::new(
            "Error deleting task",
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    Ok(())
}
