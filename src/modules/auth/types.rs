use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 characters"
    ))]
    pub username: String,

    #[validate(length(min = 8, max = 100, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: i32,
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct Claim {
    pub sub: i32,
    pub exp: usize,
}

#[derive(Clone)]
pub struct UserContext {
    pub id: i32,
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user: RegisterResponse,
    pub token: String,
}

#[derive(Serialize)]
pub struct UserProfile {
    pub id: i32,
    pub username: String,
}
