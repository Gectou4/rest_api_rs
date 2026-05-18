use axum::extract::{Path, State};
use axum::Form;
use axum::Json;
use axum::http::StatusCode;
use std::collections::HashMap;
use crate::{AppError, AppState, TaskStatus};

pub async fn create_task(
    State(state): State<AppState>,
    Form(params): Form<HashMap<String, String>>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    let title = params.get("title").ok_or(AppError::TitleRequired)?;
    let description = params.get("description").cloned().unwrap_or_default();
    let status = params
        .get("status")
        .and_then(|s| s.parse::<i32>().ok())
        .map(|s| TaskStatus::from_i8(s as i8))
        .unwrap_or(TaskStatus::Backlog);

    let task = crate::models::task::create(&state.pool, title, &description, status).await?;

    Ok((StatusCode::CREATED, Json(serde_json::to_value(task).unwrap())))
}

pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(params): Form<HashMap<String, String>>,
) -> Result<Json<i32>, AppError> {
    let title = params.get("title").cloned();
    let description = params.get("description").cloned();
    let status = params
        .get("status")
        .and_then(|s| s.parse::<i32>().ok())
        .map(|s| TaskStatus::from_i8(s as i8));

    crate::models::task::update(&state.pool, id, title, description, status).await?;

    Ok(Json(1))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<i32>, AppError> {
    crate::models::task::delete(&state.pool, id).await?;
    Ok(Json(1))
}

pub async fn add_task_to_user(
    State(state): State<AppState>,
    Path((user_id, task_id)): Path<(i32, i32)>,
) -> Result<Json<i32>, AppError> {
    crate::models::user::get_by_id(&state.pool, user_id).await?;
    crate::models::task::get_by_id(&state.pool, task_id).await?;

    crate::models::user_task::add_task_to_user(&state.pool, user_id, task_id).await?;

    Ok(Json(1))
}

pub async fn remove_task_from_user(
    State(state): State<AppState>,
    Path((user_id, task_id)): Path<(i32, i32)>,
) -> Result<Json<i32>, AppError> {
    crate::models::user::get_by_id(&state.pool, user_id).await?;
    crate::models::user_task::remove_task_from_user(&state.pool, user_id, task_id).await?;

    Ok(Json(1))
}
