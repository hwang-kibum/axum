use sqlx::MySqlPool;

use crate::{error::AppError, models::user::User};

pub async fn find_by_id(pool: &MySqlPool, id: i64) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash, created_at FROM users WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

pub async fn find_by_username(pool: &MySqlPool, username: &str) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash, created_at FROM users WHERE username = ?",
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

pub async fn create(pool: &MySqlPool, username: &str, password_hash: &str) -> Result<User, AppError> {
    let result = sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(username)
        .bind(password_hash)
        .execute(pool)
        .await?;

    find_by_id(pool, result.last_insert_id() as i64)
        .await?
        .ok_or_else(|| AppError::Internal("사용자 생성 후 조회 실패".to_string()))
}
