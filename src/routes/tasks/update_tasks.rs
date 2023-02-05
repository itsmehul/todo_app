use axum::{
    extract::{Path, State},
    Extension, Json,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::{
    database::{
        tasks::{self, Entity as Task},
        users::Model as UserModel,
    },
    utilities::app_error::AppError,
};

use super::RequestTask;

pub async fn update_task(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
    Path(task_id): Path<i32>,
    Json(request_task): Json<RequestTask>,
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

    task.priority = request_task.priority.map(Set).unwrap_or(task.priority);
    task.title = request_task.title.map(Set).unwrap_or(task.title);
    task.description = request_task
        .description
        .map(Set)
        .unwrap_or(task.description);
    task.completed_at = request_task
        .completed_at
        .map(Set)
        .unwrap_or(task.completed_at);

    task.save(&db).await.map_err(|_| {
        AppError::new(
            "Error updating task",
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    Ok(())
}

pub async fn mark_completed(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
    Path(task_id): Path<i32>,
) -> Result<(), AppError> {
    let task_to_update = Task::find()
        .filter(
            tasks::Column::Id
                .eq(task_id)
                .and(tasks::Column::UserId.eq(user.id)),
        )
        .one(&db)
        .await
        .map_err(|error| {
            AppError::new(
                "Error getting task",
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let mut task = if let Some(task_to_update) = task_to_update {
        task_to_update.into_active_model()
    } else {
        return Err(AppError::new(
            "Task not found",
            axum::http::StatusCode::NOT_FOUND,
        ));
    };

    let now = chrono::Utc::now();
    task.completed_at = Set(Some(now.into()));

    task.save(&db).await.map_err(|error| {
        AppError::new(
            "Error updating task",
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    Ok(())
}

pub async fn mark_uncompleted(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
    Path(task_id): Path<i32>,
) -> Result<(), AppError> {
    let task_to_update = Task::find()
        .filter(
            tasks::Column::Id
                .eq(task_id)
                .and(tasks::Column::UserId.eq(user.id)),
        )
        .one(&db)
        .await
        .map_err(|error| {
            AppError::new(
                "Error getting task",
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
        })?;

    let mut task = if let Some(task_to_update) = task_to_update {
        task_to_update.into_active_model()
    } else {
        return Err(AppError::new(
            "Task not found",
            axum::http::StatusCode::NOT_FOUND,
        ));
    };

    let now = chrono::Utc::now();
    task.completed_at = Set(None);

    task.save(&db).await.map_err(|error| {
        AppError::new(
            "Error updating task",
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    Ok(())
}
