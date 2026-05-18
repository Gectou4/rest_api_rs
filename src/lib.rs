use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod config;
pub mod controllers;
pub mod models;
pub mod routes;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub task_id: i32,
    pub status: TaskStatus,
    pub title: String,
    pub description: String,
    pub creation_date: DateTime<Local>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Backlog = 1,
    Todo = 2,
    InProgress = 3,
    Done = 4,
    Closed = 5,
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(*self as i32)
    }
}

impl<'de> Deserialize<'de> for TaskStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(TaskStatus::from_i8(value as i8))
    }
}

impl TaskStatus {
    pub fn from_i8(value: i8) -> Self {
        match value {
            1 => TaskStatus::Backlog,
            2 => TaskStatus::Todo,
            3 => TaskStatus::InProgress,
            4 => TaskStatus::Done,
            5 => TaskStatus::Closed,
            _ => TaskStatus::Backlog,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTask {
    pub user_id: i32,
    pub tasks: std::collections::HashMap<i32, Task>,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::MySqlPool,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("User [{0}] not exists")]
    UserNotFound(i32),
    #[error("Task [{0}] not exists")]
    TaskNotFound(i32),
    #[error("Title is required")]
    TitleRequired,
    #[error("Id of task to edit is required")]
    TaskIdRequired,
    #[error("Task not found")]
    TaskNotFoundGeneric,
    #[error("Unable to create new Task")]
    CreateTaskFailed,
    #[error("Unable to update Task")]
    UpdateTaskFailed,
    #[error("Unable to delete Task")]
    DeleteTaskFailed,
    #[error("Unable to add Task to user")]
    AddTaskToUserFailed,
    #[error("Unable to delete Task of user")]
    DeleteTaskOfUserFailed,
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            AppError::UserNotFound(_) => (axum::http::StatusCode::NOT_FOUND, self.to_string()),
            AppError::TaskNotFound(_)
            | AppError::TitleRequired
            | AppError::TaskIdRequired
            | AppError::TaskNotFoundGeneric => {
                (axum::http::StatusCode::BAD_REQUEST, self.to_string())
            }
            AppError::CreateTaskFailed
            | AppError::UpdateTaskFailed
            | AppError::DeleteTaskFailed
            | AppError::AddTaskToUserFailed
            | AppError::DeleteTaskOfUserFailed
            | AppError::Database(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                self.to_string(),
            ),
        };

        (status, axum::Json(serde_json::json!(message))).into_response()
    }
}
