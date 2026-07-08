mod error;
mod layers;
mod models;
mod modules;
mod response;
mod routes;
mod state;

use std::net::SocketAddr;

use crate::modules::auth::AuthService;
use crate::modules::task::TaskService;
use crate::state::AppState;
use axum::routing::{patch, post};
use axum::{Router, middleware};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let app_state = AppState {
        jwt_secret: jwt_secret.clone(),
        auth_service: AuthService::new(db_pool.clone(), jwt_secret.clone()),
        task_service: TaskService::new(db_pool.clone()),
    };

    let governor_layer = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(10)
        .finish()
        .expect("Could not build governor");

    let public_route = Router::new()
        .route("/api/auth/register", post(modules::auth::routes::register))
        .route("/api/auth/login", post(modules::auth::routes::login));

    let protected_route = Router::new()
        .route(
            "/api/tasks",
            post(modules::task::route::add_task).get(modules::task::route::get_all_task),
        )
        .route("/api/tasks/{id}", patch(modules::task::route::edit_task))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            layers::auth::validate_auth,
        ));

    let app = Router::new()
        .merge(public_route)
        .merge(protected_route)
        .with_state(app_state)
        .layer(GovernorLayer::new(governor_layer));

    let listener = TcpListener::bind("localhost:8000").await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
