mod error;
mod response;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::{patch, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::error::AppError;
use crate::response::ApiResponse;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Serialize)]
struct Task {
    id: i32,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct CreateTask {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTask {
    title: Option<String>,
    completed: Option<bool>,
}

async fn create_task(
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

async fn get_tasks(
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

async fn update_task(
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

async fn delete_task(
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

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    let app_state = AppState { db: db_pool };

    let app = Router::new()
        .route("/api/tasks", post(create_task).get(get_tasks))
        .route("/api/tasks/{id}", patch(update_task).delete(delete_task))
        .with_state(app_state);

    let listener = TcpListener::bind("localhost:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
