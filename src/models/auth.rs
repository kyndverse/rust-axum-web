use serde::{Deserialize, Serialize};

use crate::models::user::User;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: i32,
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct Claim {
    pub sub: String,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}
