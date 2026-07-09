use std::collections::HashMap;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

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

    #[error("Invalid token")]
    InvalidToken,

    #[error("Validation failed")]
    ValidationFailed(#[from] ValidationErrors),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
    errors: Option<HashMap<String, Vec<String>>>,
}

impl ErrorResponse {
    fn validation_errors_to_map(errors: &ValidationErrors) -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();

        for (field, field_errors) in errors.field_errors() {
            let messages = field_errors
                .iter()
                .map(|err| {
                    err.message
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| "Invalid value".to_string())
                })
                .collect();

            map.insert(field.to_string(), messages);
        }

        map
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, data) = match &self {
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: 500,
                    message: "Internal server error".to_string(),
                    errors: None,
                },
            ),
            AppError::NotFound(message) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    code: 404,
                    message: message.clone(),
                    errors: None,
                },
            ),
            AppError::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: 400,
                    message: message.clone(),
                    errors: None,
                },
            ),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: 500,
                    message: "Internal server error".to_string(),
                    errors: None,
                },
            ),
            AppError::Conflict(message) => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    code: 409,
                    message: message.clone(),
                    errors: None,
                },
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    code: 401,
                    message: "Invalid username or password".to_string(),
                    errors: None,
                },
            ),
            AppError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    code: 401,
                    message: "Invalid or missing token".to_string(),
                    errors: None,
                },
            ),
            AppError::ValidationFailed(errors) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: 400,
                    message: "Validation failed".to_string(),
                    errors: Some(ErrorResponse::validation_errors_to_map(&errors)),
                },
            ),
        };

        let body = Json(data);
        (code, body).into_response()
    }
}
