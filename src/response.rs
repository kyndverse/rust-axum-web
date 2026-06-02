use axum::{Json, http::StatusCode};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

pub type ApiResult<T> = Result<Json<ApiResponse<T>>, crate::error::AppError>;
pub type ApiCreatedResult<T> = Result<(StatusCode, Json<ApiResponse<T>>), crate::error::AppError>;
