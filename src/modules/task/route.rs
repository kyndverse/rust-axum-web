use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use super::types::{CreateTaskRequest, Task, UpdateTaskRequest};
use crate::{
    response::{ApiCreatedResult, ApiResponse, ApiResult},
    state::AppState,
};

pub async fn add_task(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateTaskRequest>,
) -> ApiCreatedResult<Task> {
    let task = app_state.task_service.add_task(payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            code: StatusCode::CREATED.as_u16(),
            message: "Create task successfully".to_string(),
            data: Some(task),
        }),
    ))
}

pub async fn get_all_task(State(app_state): State<AppState>) -> ApiResult<Vec<Task>> {
    let tasks = app_state.task_service.get_all_task().await?;

    Ok(Json(ApiResponse {
        code: StatusCode::OK.as_u16(),
        message: "Get all task successfully".to_string(),
        data: Some(tasks),
    }))
}

pub async fn edit_task(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTaskRequest>,
) -> ApiResult<Task> {
    let task = app_state.task_service.edit_task(id, payload).await?;

    Ok(Json(ApiResponse {
        code: StatusCode::OK.as_u16(),
        message: "Edit task successfully".to_string(),
        data: Some(task),
    }))
}

pub async fn delete_task(State(app_state): State<AppState>, Path(id): Path<i32>) -> ApiResult<()> {
    app_state.task_service.delete_task(id).await?;

    Ok(Json(ApiResponse {
        code: StatusCode::OK.as_u16(),
        message: "Delete task successfully".to_string(),
        data: None,
    }))
}
