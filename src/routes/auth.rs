use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    error::AppError,
    models::{
        auth::{Claim, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse},
        user::User,
    },
    response::ApiResponse,
    state::AppState,
};

pub async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<RegisterResponse>>, AppError> {
    let hashed_password =
        hash(payload.password, DEFAULT_COST).map_err(|_| AppError::InternalServerError)?;

    let user = sqlx::query_as!(RegisterResponse,
        "INSERT INTO users (username, password) VALUES ($1, $2) ON CONFLICT (username) DO NOTHING RETURNING id, username",
        payload.username,
        hashed_password
    ).fetch_optional(&app_state.db).await?;

    match user {
        Some(user) => Ok(Json(ApiResponse {
            code: StatusCode::CREATED.as_u16(),
            message: "User registered successfully".to_string(),
            data: Some(RegisterResponse {
                id: user.id,
                username: user.username,
            }),
        })),
        None => Err(AppError::Conflict("Username already exists".to_string())),
    }
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    let user = sqlx::query!(
        "SELECT id, username, password from users WHERE username = $1",
        payload.username
    )
    .fetch_optional(&app_state.db)
    .await?;

    let user = user.ok_or(AppError::Unauthorized)?;

    let is_valid =
        verify(payload.password, &user.password).map_err(|_| AppError::InternalServerError)?;

    if !is_valid {
        return Err(AppError::Unauthorized);
    }

    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(10))
        .ok_or(AppError::Unauthorized)?;

    let claims = Claim {
        sub: user.username.clone(),
        exp: expiration.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            std::env::var("JWT_SECRET")
                .expect("JWT_SECRET not found")
                .as_bytes(),
        ),
    )
    .map_err(|_| AppError::InternalServerError)?;

    Ok(Json(ApiResponse {
        code: 200,
        message: "Login successfully!".to_string(),
        data: Some(LoginResponse {
            user: User {
                id: user.id,
                username: user.username,
            },
            token,
        }),
    }))
}
