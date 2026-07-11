use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::PgPool;

use super::types::{Claims, LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use crate::{error::AppError, modules::auth::types::UserProfile};

use super::repository::AuthRepository;

#[derive(Clone)]
pub struct AuthService {
    repo: AuthRepository,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(db: PgPool, jwt_secret: String) -> Self {
        Self {
            repo: AuthRepository::new(db),
            jwt_secret,
        }
    }

    pub async fn register(&self, payload: RegisterRequest) -> Result<RegisterResponse, AppError> {
        let hashed_password =
            hash(payload.password, DEFAULT_COST).map_err(|_| AppError::InternalServerError)?;

        let user = self
            .repo
            .create_user(RegisterRequest {
                username: payload.username,
                password: hashed_password,
            })
            .await?;

        match user {
            Some(user) => Ok(user),
            None => Err(AppError::Conflict("Username already exists".to_string())),
        }
    }

    pub async fn login(&self, payload: LoginRequest) -> Result<LoginResponse, AppError> {
        let user = self.repo.get_user_by_username(payload.username).await?;
        let user = user.ok_or(AppError::Unauthorized)?;

        let is_valid =
            tokio::task::spawn_blocking(move || verify(payload.password, &user.password))
                .await
                .map_err(|_| AppError::InternalServerError)?
                .map_err(|_| AppError::InternalServerError)?;

        if !is_valid {
            return Err(AppError::Unauthorized);
        }

        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(10))
            .ok_or(AppError::InternalServerError)?;

        let claims = Claims {
            sub: user.id,
            exp: expiration.timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.jwt_secret.as_ref()),
        )
        .map_err(|_| AppError::InternalServerError)?;

        Ok(LoginResponse {
            user: RegisterResponse {
                id: user.id,
                username: user.username,
            },
            token,
        })
    }

    pub async fn get_profile(&self, id: i32) -> Result<UserProfile, AppError> {
        let user = self
            .repo
            .get_user_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        Ok(UserProfile {
            id: user.id,
            username: user.username,
        })
    }
}
