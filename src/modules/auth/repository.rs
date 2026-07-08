use sqlx::PgPool;

use super::types::{RegisterRequest, RegisterResponse};
use crate::{error::AppError, models::user::User};

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

    pub async fn get_user_by_username(&self, username: String) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, username, password from users WHERE username = $1",
            username
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }
}
