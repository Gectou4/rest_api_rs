use crate::{AppError, User};
use sqlx::MySqlPool;

pub async fn get_by_id(pool: &MySqlPool, id: u32) -> Result<User, AppError> {
    let row = sqlx::query_as::<_, (u32, String, String)>(
        "SELECT user_id, name, email FROM user WHERE user_id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some((user_id, name, email)) => Ok(User {
            user_id,
            name,
            email,
        }),
        None => Err(AppError::UserNotFound(id)),
    }
}
