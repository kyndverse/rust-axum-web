use sqlx::PgPool;

use super::types::{RegisterRequest, RegisterResponse};
use crate::error::AppError;

#[derive(Clone)]
pub struct AuthRepository {
    db: PgPool,
}

impl AuthRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn create_user(
        &self,
        data: RegisterRequest,
    ) -> Result<Option<RegisterResponse>, AppError> {
        let user = sqlx::query_as!(RegisterResponse,
        "INSERT INTO users (username, password) VALUES ($1, $2) ON CONFLICT (username) DO NOTHING RETURNING id, username",
        data.username,
        data.password
      ).fetch_optional(&self.db).await?;

        Ok(user)
    }
}
