use sqlx::PgPool;

use crate::modules::auth::AuthService;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub auth_service: AuthService,
}
