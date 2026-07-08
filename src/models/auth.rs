use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Claim {
    pub sub: i32,
    pub exp: usize,
}

#[derive(Clone)]
pub struct UserContext {
    pub id: i32,
}
