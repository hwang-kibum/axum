use sqlx::MySqlPool;

use crate::{error::AppError, models::post::Post};

const SELECT_POST: &str = "
    SELECT p.id, p.title, p.content, p.author_id,
           u.username AS author_name, p.created_at, p.updated_at
    FROM posts p
    JOIN users u ON p.author_id = u.id
";

pub async fn find_by_id(pool: &MySqlPool, id: i64) -> Result<Option<Post>, AppError> {
    let sql = format!("{SELECT_POST} WHERE p.id = ?");
    let post = sqlx::query_as::<_, Post>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(post)
}

pub async fn list(pool: &MySqlPool, page: i64, per_page: i64) -> Result<(Vec<Post>, i64), AppError> {
    let offset = (page - 1) * per_page;
    let sql = format!("{SELECT_POST} ORDER BY p.created_at DESC LIMIT ? OFFSET ?");

    let posts = sqlx::query_as::<_, Post>(&sql)
        .bind(per_page)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts")
        .fetch_one(pool)
        .await?;

    Ok((posts, total))
}

pub async fn create(
    pool: &MySqlPool,
    title: &str,
    content: &str,
    author_id: i64,
) -> Result<Post, AppError> {
    let result = sqlx::query("INSERT INTO posts (title, content, author_id) VALUES (?, ?, ?)")
        .bind(title)
        .bind(content)
        .bind(author_id)
        .execute(pool)
        .await?;

    find_by_id(pool, result.last_insert_id() as i64)
        .await?
        .ok_or_else(|| AppError::Internal("게시글 생성 후 조회 실패".to_string()))
}

pub async fn update(
    pool: &MySqlPool,
    id: i64,
    title: &str,
    content: &str,
) -> Result<Post, AppError> {
    sqlx::query("UPDATE posts SET title = ?, content = ? WHERE id = ?")
        .bind(title)
        .bind(content)
        .bind(id)
        .execute(pool)
        .await?;

    find_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("게시글을 찾을 수 없습니다".to_string()))
}

pub async fn delete(pool: &MySqlPool, id: i64) -> Result<(), AppError> {
    sqlx::query("DELETE FROM posts WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
