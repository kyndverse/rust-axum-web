use serde::{Deserialize, Serialize};

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
