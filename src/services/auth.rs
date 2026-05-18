use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::MySqlPool;

use crate::{
    error::AppError,
    models::user::{AuthResponse, Claims, LoginRequest, RegisterRequest},
    repositories::user as user_repo,
};

pub async fn register(
    pool: &MySqlPool,
    jwt_secret: &str,
    req: RegisterRequest,
) -> Result<AuthResponse, AppError> {
    if req.username.trim().is_empty() || req.password.len() < 8 {
        return Err(AppError::BadRequest(
            "사용자 이름은 필수이며 비밀번호는 8자 이상이어야 합니다".to_string(),
        ));
    }

    if user_repo::find_by_username(pool, &req.username).await?.is_some() {
        return Err(AppError::BadRequest("이미 사용 중인 사용자 이름입니다".to_string()));
    }

    // bcrypt는 CPU 집약적 작업이므로 블로킹 스레드에서 실행
    let password = req.password.clone();
    let hash = tokio::task::spawn_blocking(move || bcrypt::hash(password, bcrypt::DEFAULT_COST))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let user = user_repo::create(pool, &req.username, &hash).await?;
    let token = make_token(user.id, &user.username, jwt_secret)?;

    Ok(AuthResponse { token, user_id: user.id, username: user.username })
}

pub async fn login(
    pool: &MySqlPool,
    jwt_secret: &str,
    req: LoginRequest,
) -> Result<AuthResponse, AppError> {
    let user = user_repo::find_by_username(pool, &req.username)
        .await?
        .ok_or_else(|| AppError::Unauthorized("사용자 이름 또는 비밀번호가 올바르지 않습니다".to_string()))?;

    let hash = user.password_hash.clone();
    let password = req.password;
    let valid = tokio::task::spawn_blocking(move || bcrypt::verify(password, &hash))
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if !valid {
        return Err(AppError::Unauthorized(
            "사용자 이름 또는 비밀번호가 올바르지 않습니다".to_string(),
        ));
    }

    let token = make_token(user.id, &user.username, jwt_secret)?;

    Ok(AuthResponse { token, user_id: user.id, username: user.username })
}

fn make_token(user_id: i64, username: &str, secret: &str) -> Result<String, AppError> {
    let exp = (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;
    let claims = Claims { sub: user_id, username: username.to_string(), exp };
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))?;
    Ok(token)
}
