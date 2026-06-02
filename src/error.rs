use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error")]
    InternalServerError,

    #[error("Unauthorized")]
    Unauthorized,
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, data) = match &self {
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: 500,
                    message: "Internal server error".to_string(),
                },
            ),
            AppError::NotFound(message) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    code: 404,
                    message: message.clone(),
                },
            ),
            AppError::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: 400,
                    message: message.clone(),
                },
            ),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: 500,
                    message: "Internal server error".to_string(),
                },
            ),
            AppError::Conflict(message) => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    code: 409,
                    message: message.clone(),
                },
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    code: 401,
                    message: "Invalid username or password".to_string(),
                },
            ),
        };

        let body = Json(data);
        (code, body).into_response()
    }
}
