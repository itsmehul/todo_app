use axum::{
    extract::{Path, State},
    Extension, Json,
};
use sea_orm::DatabaseConnection;

use crate::{queries::task_queries, utilities::app_error::AppError};

use crate::database::users::Model as UserModel;

use super::{ResponseDataTask, ResponseTask};

pub async fn get_one_task(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
    Path(task_id): Path<i32>,
) -> Result<Json<ResponseDataTask>, AppError> {
    let task = task_queries::find_task_by_id(&db, task_id, user.id).await?;

    let response_task = ResponseTask {
        id: task.id,
        title: task.title,
        description: task.description,
        priority: task.priority,
        completed_at: task
            .completed_at
            .map(|completed_at| completed_at.to_string()),
    };

    Ok(Json(ResponseDataTask {
        data: response_task,
    }))
}
