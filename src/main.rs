use axum::response::IntoResponse;
use axum::{ Json, Router };
use axum::extract::{ Path, Query };
use axum::routing::{ get, post };
use serde::{ Deserialize, Serialize };
use tokio::net::TcpListener;

async fn detail_user(Path(id): Path<String>) -> Json<User> {
    Json(User {
        id,
        name: String::from("Eko"),
        email: String::from("eko@gmail.com"),
    })
}

#[derive(Deserialize)]
struct Pagination {
    page: u32,
    size: u32,
}

async fn index_user(Query(pagination): Query<Pagination>) -> String {
    format!("Menampilan dara user di page: {} dengan size {}", pagination.page, pagination.size)
}

#[derive(Deserialize, Serialize)]
struct User {
    id: String,
    name: String,
    email: String,
}

async fn create_user(Json(paylaod): Json<User>) -> impl IntoResponse {
    format!("User created with Username: {} and email: {}", paylaod.name, paylaod.email)
} // Cek ulang

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/users", get(index_user))
        .route("/api/users/{id}", get(detail_user))
        .route("/api/users", post(create_user));

    let listener = TcpListener::bind("localhost:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
