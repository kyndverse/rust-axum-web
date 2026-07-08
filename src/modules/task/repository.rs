use crate::error::AppError;

use super::types::{CreateTaskRequest, Task, UpdateTaskRequest};
use sqlx::PgPool;

#[derive(Clone)]
pub struct TaskRepository {
    db: PgPool,
}

impl TaskRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn create_task(&self, data: CreateTaskRequest) -> Result<Task, AppError> {
        let task = sqlx::query_as!(
            Task,
            "INSERT INTO tasks (title) VALUES ($1) RETURNING id, title, completed",
            &data.title
        )
        .fetch_one(&self.db)
        .await?;

        Ok(task)
    }

    pub async fn get_tasks(&self) -> Result<Vec<Task>, AppError> {
        let tasks = sqlx::query_as!(Task, "SELECT * FROM tasks")
            .fetch_all(&self.db)
            .await?;

        Ok(tasks)
    }

    pub async fn update_task_by_id(
        &self,
        id: i32,
        data: UpdateTaskRequest,
    ) -> Result<Option<Task>, AppError> {
        let task = sqlx::query_as!(
            Task,
            "
    UPDATE tasks
    SET
        title = COALESCE($1, title),
        completed = COALESCE($2, completed)
    WHERE id = $3
    RETURNING id, title, completed
    ",
            data.title,
            data.completed,
            id
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(task)
    }

    // pub async fn delete_task_by_id(&self, id: i32) -> Result<String, AppError> {
        
    // }
}
