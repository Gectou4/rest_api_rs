use axum::extract::{Path, State};
use axum::Json;
use crate::{AppError, AppState};

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = crate::models::user::get_by_id(&state.pool, id).await?;
    Ok(Json(serde_json::to_value(user).unwrap()))
}

pub async fn get_user_tasks(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user_tasks = crate::models::user_task::get_tasks_by_user(&state.pool, id).await?;
    Ok(Json(serde_json::to_value(user_tasks).unwrap()))
}
