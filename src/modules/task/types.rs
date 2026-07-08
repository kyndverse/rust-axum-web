use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub completed: Option<bool>,
}
