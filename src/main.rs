mod error;
mod models;
mod response;
mod routes;
mod state;

use crate::state::AppState;
use axum::Router;
use axum::routing::{patch, post};
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    let app_state = AppState { db: db_pool };

    let app = Router::new()
        .route(
            "/api/tasks",
            post(routes::task::create_task).get(routes::task::get_tasks),
        )
        .route(
            "/api/tasks/{id}",
            patch(routes::task::update_task).delete(routes::task::delete_task),
        )
        .route("/api/auth/register", post(routes::auth::register))
        .route("/api/auth/login", post(routes::auth::login))
        .with_state(app_state);

    let listener = TcpListener::bind("localhost:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
