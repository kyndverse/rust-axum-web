use bcrypt::{DEFAULT_COST, hash};
use sqlx::PgPool;

use super::types::{RegisterRequest, RegisterResponse};
use crate::error::AppError;

use super::repository::AuthRepository;

#[derive(Clone)]

pub struct AuthService {
    repo: AuthRepository,
}

impl AuthService {
    pub fn new(db: PgPool) -> Self {
        Self {
            repo: AuthRepository::new(db),
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
}
