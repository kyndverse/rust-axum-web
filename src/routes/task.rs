// use axum::{
//     Json,
//     extract::{Path, State},
//     http::StatusCode,
// };

// use crate::{
//     AppState,
//     error::AppError,
//     models::task::Task,
//     response::{ApiResponse, ApiResult},
// };

// pub async fn delete_task(
//     State(app_state): State<AppState>,
//     Path(id): Path<i32>,
// ) -> ApiResult<Task> {
//     let deteted = sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
//         .execute(&app_state.db)
//         .await?;

//     if deteted.rows_affected() > 0 {
//         Ok(Json(ApiResponse {
//             code: StatusCode::OK.as_u16(),
//             message: "Delete task successfully!".to_string(),
//             data: None,
//         }))
//     } else {
//         Err(AppError::NotFound("Task not found".to_string()))
//     }
// }
