use crate::{
    models::auth::UserContext,
    modules::auth::types::UserProfile,
    response::{ApiCreatedResult, ApiResponse, ApiResult},
    state::AppState,
};
use axum::{Extension, Json, extract::State, http::StatusCode};

use super::types::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};

pub async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> ApiCreatedResult<RegisterResponse> {
    let result = app_state.auth_service.register(payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            code: StatusCode::CREATED.as_u16(),
            message: "User registered successfully".to_string(),
            data: Some(RegisterResponse {
                id: result.id,
                username: result.username,
            }),
        }),
    ))
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> ApiResult<LoginResponse> {
    let result = app_state.auth_service.login(payload).await?;

    Ok(Json(ApiResponse {
        code: StatusCode::OK.as_u16(),
        message: "Login successfully".to_string(),
        data: Some(result),
    }))
}

pub async fn get_profile(
    State(app_state): State<AppState>,
    Extension(user_ctx): Extension<UserContext>,
) -> ApiResult<UserProfile> {
    let user = app_state.auth_service.get_profile(user_ctx.id).await?;

    Ok(Json(ApiResponse {
        code: StatusCode::OK.as_u16(),
        message: "Get profile successfully".to_string(),
        data: Some(user),
    }))
}
