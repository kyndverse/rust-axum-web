use serde::Serialize;

#[derive(Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
