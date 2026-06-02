use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    AppState,
    error::AppError,
    models::task::{CreateTask, Task, UpdateTask},
    response::ApiResponse,
};

pub async fn create_task(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> Result<(StatusCode, Json<ApiResponse<Task>>), AppError> {
    let data_result = sqlx::query_as!(
        Task,
        "INSERT INTO tasks (title) VALUES ($1) RETURNING id, title, completed",
        payload.title
    )
    .fetch_one(&app_state.db)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            code: StatusCode::CREATED.as_u16(),
            message: "Task created successfully!".to_string(),
            data: Some(data_result),
        }),
    ))
}

pub async fn get_tasks(
    State(app_state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<Task>>>, AppError> {
    let data_result = sqlx::query_as!(Task, "SELECT * FROM tasks")
        .fetch_all(&app_state.db)
        .await?;

    Ok(Json(ApiResponse {
        code: StatusCode::OK.as_u16(),
        message: "List task".to_string(),
        data: Some(data_result),
    }))
}

pub async fn update_task(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<ApiResponse<Task>>, AppError> {
    let task = sqlx::query_as!(
        Task,
        "
    UPDATE tasks
    SET
        title = COALESCE($1, title),
        completed = COALESCE($2, completed)
    WHERE id = $3
    RETURNING id, title, completed
    ",
        payload.title,
        payload.completed,
        id
    )
    .fetch_optional(&app_state.db)
    .await?;

    match task {
        Some(task) => Ok(Json(ApiResponse {
            code: StatusCode::OK.as_u16(),
            message: "Update task successfully!".to_string(),
            data: Some(task),
        })),
        None => Err(AppError::NotFound("Task not found".to_string())),
    }
}

pub async fn delete_task(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let deteted = sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
        .execute(&app_state.db)
        .await?;

    if deteted.rows_affected() > 0 {
        Ok(Json(ApiResponse {
            code: StatusCode::OK.as_u16(),
            message: "Delete task successfully!".to_string(),
            data: None,
        }))
    } else {
        Err(AppError::NotFound("Task not found".to_string()))
    }
}
