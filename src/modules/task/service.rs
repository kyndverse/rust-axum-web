use super::types::{CreateTaskRequest, Task, UpdateTaskRequest};
use crate::{error::AppError, modules::task::repository::TaskRepository};
use sqlx::PgPool;

#[derive(Clone)]
pub struct TaskService {
    repo: TaskRepository,
}

impl TaskService {
    pub fn new(db: PgPool) -> Self {
        Self {
            repo: TaskRepository::new(db),
        }
    }

    pub async fn add_task(&self, data: CreateTaskRequest) -> Result<Task, AppError> {
        let task = self.repo.create_task(data).await?;
        Ok(task)
    }

    pub async fn get_all_task(&self) -> Result<Vec<Task>, AppError> {
        let tasks = self.repo.get_tasks().await?;
        Ok(tasks)
    }

    pub async fn edit_task(&self, id: i32, data: UpdateTaskRequest) -> Result<Task, AppError> {
        let task = self.repo.update_task_by_id(id, data).await?;

        match task {
            Some(task) => Ok(task),
            None => Err(AppError::NotFound("Task not found".to_string())),
        }
    }

    // pub async fn delete_task(&self, id: i32) -> Result<String, AppError> {

    // }
}
