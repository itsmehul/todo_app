use axum::Extension;
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::database::users::Model as UserModel;

use crate::queries::task_queries;
use crate::utilities::app_error::AppError;

use super::{ResponseDataTask, ResponseDataTasks, ResponseTask};

pub async fn get_all_tasks(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
) -> Result<Json<ResponseDataTasks>, AppError> {
    let user_id = user.id;

    let tasks = task_queries::get_all_tasks(&db, user_id, false)
        .await?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            title: db_task.title,
            description: db_task.description,
            priority: db_task.priority,
            completed_at: db_task
                .completed_at
                .map(|completed_at| completed_at.to_string()),
        })
        .collect::<Vec<ResponseTask>>();

    Ok(Json(ResponseDataTasks { data: tasks }))
}
