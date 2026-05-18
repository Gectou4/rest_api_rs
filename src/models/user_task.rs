use chrono::Local;
use sqlx::MySqlPool;
use crate::{AppError, Task, TaskStatus, UserTask};

pub async fn get_tasks_by_user(pool: &MySqlPool, user_id: i32) -> Result<UserTask, AppError> {
    let rows = sqlx::query_as::<_, (i32, i32, String, String, chrono::NaiveDateTime)>(
        r#"
        SELECT t.task_id, t.status, t.title, t.description, t.creation_date
        FROM task t
        INNER JOIN user_task ut ON t.task_id = ut.task_id
        WHERE ut.user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let mut task_map = std::collections::HashMap::new();
    for (task_id, status, title, description, creation_date) in rows {
        let task = Task {
            task_id,
            status: TaskStatus::from_i8(status as i8),
            title,
            description,
            creation_date: creation_date.and_local_timezone(Local).single().unwrap(),
        };
        task_map.insert(task.task_id, task);
    }

    Ok(UserTask {
        user_id,
        tasks: task_map,
    })
}

pub async fn add_task_to_user(
    pool: &MySqlPool,
    user_id: i32,
    task_id: i32,
) -> Result<(), AppError> {
    sqlx::query("INSERT INTO user_task (user_id, task_id) VALUES (?, ?)")
        .bind(user_id)
        .bind(task_id)
        .execute(pool)
        .await
        .map_err(|_| AppError::AddTaskToUserFailed)?;

    Ok(())
}

pub async fn remove_task_from_user(
    pool: &MySqlPool,
    user_id: i32,
    task_id: i32,
) -> Result<(), AppError> {
    sqlx::query("DELETE FROM user_task WHERE user_id = ? AND task_id = ?")
        .bind(user_id)
        .bind(task_id)
        .execute(pool)
        .await
        .map_err(|_| AppError::DeleteTaskOfUserFailed)?;

    Ok(())
}
