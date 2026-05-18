use axum::routing::{delete, get, post};
use axum::Router;
use crate::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/user/{id}", get(crate::controllers::user::get_user))
        .route(
            "/user/{id}/task",
            get(crate::controllers::user::get_user_tasks),
        )
        .route("/task", post(crate::controllers::task::create_task))
        .route(
            "/task/{id}",
            post(crate::controllers::task::update_task),
        )
        .route(
            "/task/{id}",
            delete(crate::controllers::task::delete_task),
        )
        .route(
            "/user/{id}/task/{task_id}",
            post(crate::controllers::task::add_task_to_user),
        )
        .route(
            "/user/{id}/task/{task_id}",
            delete(crate::controllers::task::remove_task_from_user),
        )
        .with_state(state)
}
