mod error;
mod layers;
mod models;
mod response;
mod routes;
mod state;

use crate::state::AppState;
use axum::routing::{get, patch, post};
use axum::{Router, middleware};
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let app_state = AppState {
        db: db_pool,
        jwt_secret,
    };

    let public_route = Router::new().route("/api/auth/login", post(routes::auth::login));

    let protected_route = Router::new()
        .route(
            "/api/tasks",
            post(routes::task::create_task).get(routes::task::get_tasks),
        )
        .route(
            "/api/tasks/{id}",
            patch(routes::task::update_task).delete(routes::task::delete_task),
        )
        .route("/api/auth/register", post(routes::auth::register))
        .route("/api/auth/me", get(routes::auth::profile))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            layers::auth::validate_auth,
        ));

    let app = Router::new()
        .merge(public_route)
        .merge(protected_route)
        .with_state(app_state);

    let listener = TcpListener::bind("localhost:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
