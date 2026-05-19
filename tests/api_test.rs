use g4api::AppState;
use sqlx::MySqlPool;

async fn setup_test_pool() -> MySqlPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:root@localhost:3306/rest_api".to_string());
    sqlx::MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

#[tokio::test]
async fn test_get_user() {
    let pool = setup_test_pool().await;
    let state = AppState { pool };

    let user = g4api::models::user::get_by_id(&state.pool, 1).await;
    assert!(user.is_ok());
    let user = user.unwrap();
    assert_eq!(user.user_id, 1);
    assert_eq!(user.name, "G4");
    assert_eq!(user.email, "gectou4@gmail.com");
}

#[tokio::test]
async fn test_get_user_not_found() {
    let pool = setup_test_pool().await;
    let state = AppState { pool };

    let result = g4api::models::user::get_by_id(&state.pool, 999).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_user_tasks() {
    let pool = setup_test_pool().await;
    let state = AppState { pool };

    let user_tasks = g4api::models::user_task::get_tasks_by_user(&state.pool, 1).await;
    assert!(user_tasks.is_ok());
    let user_tasks = user_tasks.unwrap();
    assert_eq!(user_tasks.user_id, 1);
    assert!(!user_tasks.tasks.is_empty());
}

#[tokio::test]
async fn test_create_task() {
    let pool = setup_test_pool().await;
    let state = AppState { pool };

    let task = g4api::models::task::create(
        &state.pool,
        "Test task",
        "Test description",
        g4api::TaskStatus::Backlog,
    )
    .await;
    assert!(task.is_ok());
    let task = task.unwrap();
    assert!(task.task_id > 0);
    assert_eq!(task.title, "Test task");

    g4api::models::task::delete(&state.pool, task.task_id)
        .await
        .expect("Failed to clean up test task");
}

#[tokio::test]
async fn test_update_task() {
    let pool = setup_test_pool().await;
    let state = AppState { pool };

    let task = g4api::models::task::create(
        &state.pool,
        "Original title",
        "Original description",
        g4api::TaskStatus::Backlog,
    )
    .await
    .expect("Failed to create test task");

    let result = g4api::models::task::update(
        &state.pool,
        task.task_id,
        Some("Updated title".to_string()),
        None,
        None,
    )
    .await;
    assert!(result.is_ok());

    let updated = g4api::models::task::get_by_id(&state.pool, task.task_id)
        .await
        .expect("Failed to fetch updated task");
    assert_eq!(updated.title, "Updated title");

    g4api::models::task::delete(&state.pool, task.task_id)
        .await
        .expect("Failed to clean up test task");
}

#[tokio::test]
async fn test_delete_task() {
    let pool = setup_test_pool().await;
    let state = AppState { pool };

    let task = g4api::models::task::create(
        &state.pool,
        "To delete",
        "This will be deleted",
        g4api::TaskStatus::Todo,
    )
    .await
    .expect("Failed to create test task");

    let result = g4api::models::task::delete(&state.pool, task.task_id).await;
    assert!(result.is_ok());

    let result = g4api::models::task::get_by_id(&state.pool, task.task_id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_add_task_to_user() {
    let pool = setup_test_pool().await;
    let state = AppState { pool };

    let task = g4api::models::task::create(
        &state.pool,
        "Task for user",
        "Associated task",
        g4api::TaskStatus::Backlog,
    )
    .await
    .expect("Failed to create test task");

    let result = g4api::models::user_task::add_task_to_user(&state.pool, 1, task.task_id).await;
    assert!(result.is_ok());

    let user_tasks = g4api::models::user_task::get_tasks_by_user(&state.pool, 1)
        .await
        .expect("Failed to get user tasks");
    assert!(user_tasks.tasks.contains_key(&task.task_id));

    g4api::models::user_task::remove_task_from_user(&state.pool, 1, task.task_id)
        .await
        .expect("Failed to remove task from user");
    g4api::models::task::delete(&state.pool, task.task_id)
        .await
        .expect("Failed to clean up test task");
}

#[tokio::test]
async fn test_remove_task_from_user() {
    let pool = setup_test_pool().await;
    let state = AppState { pool };

    let task = g4api::models::task::create(
        &state.pool,
        "Task to remove",
        "Will be unlinked",
        g4api::TaskStatus::Backlog,
    )
    .await
    .expect("Failed to create test task");

    g4api::models::user_task::add_task_to_user(&state.pool, 1, task.task_id)
        .await
        .expect("Failed to add task to user");

    let result =
        g4api::models::user_task::remove_task_from_user(&state.pool, 1, task.task_id).await;
    assert!(result.is_ok());

    let user_tasks = g4api::models::user_task::get_tasks_by_user(&state.pool, 1)
        .await
        .expect("Failed to get user tasks");
    assert!(!user_tasks.tasks.contains_key(&task.task_id));

    g4api::models::task::delete(&state.pool, task.task_id)
        .await
        .expect("Failed to clean up test task");
}
