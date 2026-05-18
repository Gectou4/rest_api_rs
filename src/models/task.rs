use chrono::Local;
use sqlx::MySqlPool;
use crate::{AppError, Task, TaskStatus};

pub async fn get_by_id(pool: &MySqlPool, id: i32) -> Result<Task, AppError> {
    let row = sqlx::query_as::<_, (i32, i32, String, String, chrono::NaiveDateTime)>(
        "SELECT task_id, status, title, description, creation_date FROM task WHERE task_id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some((task_id, status, title, description, creation_date)) => Ok(Task {
            task_id,
            status: TaskStatus::from_i8(status as i8),
            title,
            description,
            creation_date: creation_date.and_local_timezone(Local).single().unwrap(),
        }),
        None => Err(AppError::TaskNotFound(id)),
    }
}

pub async fn create(
    pool: &MySqlPool,
    title: &str,
    description: &str,
    status: TaskStatus,
) -> Result<Task, AppError> {
    let creation_date = chrono::Local::now();

    let result = sqlx::query(
        "INSERT INTO task (status, title, description, creation_date) VALUES (?, ?, ?, ?)",
    )
    .bind(status as i32)
    .bind(title)
    .bind(description)
    .bind(creation_date.format("%Y-%m-%d %H:%M:%S").to_string())
    .execute(pool)
    .await;

    match result {
        Ok(res) => {
            let task_id = res.last_insert_id() as i32;
            Ok(Task {
                task_id,
                status,
                title: title.to_string(),
                description: description.to_string(),
                creation_date,
            })
        }
        Err(_) => Err(AppError::CreateTaskFailed),
    }
}

pub async fn update(
    pool: &MySqlPool,
    id: i32,
    title: Option<String>,
    description: Option<String>,
    status: Option<TaskStatus>,
) -> Result<(), AppError> {
    let task = get_by_id(pool, id).await?;

    let new_title = title.unwrap_or(task.title);
    let new_desc = description.unwrap_or(task.description);
    let new_status = status.unwrap_or(task.status);

    sqlx::query(
        "UPDATE task SET status = ?, title = ?, description = ? WHERE task_id = ?",
    )
    .bind(new_status as i32)
    .bind(new_title)
    .bind(new_desc)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|_| AppError::UpdateTaskFailed)?;

    Ok(())
}

pub async fn delete(pool: &MySqlPool, id: i32) -> Result<(), AppError> {
    get_by_id(pool, id).await?;

    sqlx::query("DELETE FROM task WHERE task_id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|_| AppError::DeleteTaskFailed)?;

    Ok(())
}

pub async fn get_all(pool: &MySqlPool) -> Result<Vec<Task>, AppError> {
    let rows = sqlx::query_as::<_, (i32, i32, String, String, chrono::NaiveDateTime)>(
        "SELECT task_id, status, title, description, creation_date FROM task",
    )
    .fetch_all(pool)
    .await?;

    let tasks = rows
        .into_iter()
        .map(|(task_id, status, title, description, creation_date)| Task {
            task_id,
            status: TaskStatus::from_i8(status as i8),
            title,
            description,
            creation_date: creation_date.and_local_timezone(Local).single().unwrap(),
        })
        .collect();

    Ok(tasks)
}
