use crate::AppState;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};

async fn health() -> Json<&'static str> {
    Json("ok")
}

async fn test_route() -> impl IntoResponse {
    (StatusCode::OK, "test route works")
}

async fn echo_id(Path(id): Path<String>) -> String {
    format!("echo: {}", id)
}

async fn fallback(uri: axum::http::Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/test", get(test_route))
        .route("/echo/:id", get(echo_id))
        .route("/user/:id", get(crate::controllers::user::get_user))
        .route(
            "/user/:id/task",
            get(crate::controllers::user::get_user_tasks),
        )
        .route("/task", post(crate::controllers::task::create_task))
        .route("/task/:id", post(crate::controllers::task::update_task))
        .route("/task/:id", delete(crate::controllers::task::delete_task))
        .route(
            "/user/:id/task/:task_id",
            post(crate::controllers::task::add_task_to_user),
        )
        .route(
            "/user/:id/task/:task_id",
            delete(crate::controllers::task::remove_task_from_user),
        )
        .fallback(fallback)
        .with_state(state)
}
