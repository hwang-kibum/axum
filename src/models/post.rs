use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub author_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct PostQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PostListResponse {
    pub posts: Vec<Post>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}
