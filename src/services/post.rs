use sqlx::MySqlPool;

use crate::{
    error::AppError,
    models::post::{CreatePostRequest, Post, PostListResponse, PostQuery, UpdatePostRequest},
    repositories::post as post_repo,
};

pub async fn list(pool: &MySqlPool, query: PostQuery) -> Result<PostListResponse, AppError> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);

    let (posts, total) = post_repo::list(pool, page, per_page).await?;

    Ok(PostListResponse { posts, total, page, per_page })
}

pub async fn get(pool: &MySqlPool, id: i64) -> Result<Post, AppError> {
    post_repo::find_by_id(pool, id)
        .await?
        .ok_or_else(|| AppError::NotFound("게시글을 찾을 수 없습니다".to_string()))
}

pub async fn create(
    pool: &MySqlPool,
    req: CreatePostRequest,
    author_id: i64,
) -> Result<Post, AppError> {
    if req.title.trim().is_empty() {
        return Err(AppError::BadRequest("제목을 입력해주세요".to_string()));
    }
    post_repo::create(pool, &req.title, &req.content, author_id).await
}

pub async fn update(
    pool: &MySqlPool,
    id: i64,
    req: UpdatePostRequest,
    user_id: i64,
) -> Result<Post, AppError> {
    let post = get(pool, id).await?;

    if post.author_id != user_id {
        return Err(AppError::Forbidden);
    }
    if req.title.trim().is_empty() {
        return Err(AppError::BadRequest("제목을 입력해주세요".to_string()));
    }

    post_repo::update(pool, id, &req.title, &req.content).await
}

pub async fn delete(pool: &MySqlPool, id: i64, user_id: i64) -> Result<(), AppError> {
    let post = get(pool, id).await?;

    if post.author_id != user_id {
        return Err(AppError::Forbidden);
    }

    post_repo::delete(pool, id).await
}
