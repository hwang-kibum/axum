use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: i64,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub username: String,
    pub exp: usize,
}
