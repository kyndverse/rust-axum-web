use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::net::TcpListener;

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

async fn create_task(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let data_result = sqlx::query_as!(
        Task,
        "INSERT INTO tasks (title) VALUES ($1) RETURNING id, title, completed",
        payload.title
    )
    .fetch_one(&app_state.db)
    .await;

    match data_result {
        Ok(task) => Ok(Json(task)),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Failed to create task"),
        )),
    }
}

async fn get_tasks(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    let data_result = sqlx::query_as!(Task, "SELECT * FROM tasks")
        .fetch_all(&app_state.db)
        .await;

    match data_result {
        Ok(tasks) => Ok(Json(tasks)),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("Failed to get task"),
        )),
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
        .route("/api/task", post(create_task).get(get_tasks))
        .with_state(app_state);

    let listener = TcpListener::bind("localhost:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
