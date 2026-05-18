use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("인증 실패: {0}")]
    Unauthorized(String),
    #[error("권한 없음")]
    Forbidden,
    #[error("찾을 수 없음: {0}")]
    NotFound(String),
    #[error("잘못된 요청: {0}")]
    BadRequest(String),
    #[error("데이터베이스 오류")]
    Database(#[from] sqlx::Error),
    #[error("JWT 오류")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("내부 오류: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message): (StatusCode, String) = match self {
            AppError::Unauthorized(m) => (StatusCode::UNAUTHORIZED, m),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "권한이 없습니다".to_string()),
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, m),
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            AppError::Database(e) => {
                tracing::error!("DB 오류: {e}");
                (StatusCode::INTERNAL_SERVER_ERROR, "데이터베이스 오류가 발생했습니다".to_string())
            }
            AppError::Jwt(e) => {
                tracing::warn!("JWT 오류: {e}");
                (StatusCode::UNAUTHORIZED, "유효하지 않은 토큰입니다".to_string())
            }
            AppError::Internal(m) => {
                tracing::error!("내부 오류: {m}");
                (StatusCode::INTERNAL_SERVER_ERROR, "내부 서버 오류가 발생했습니다".to_string())
            }
        };
        (status, Json(json!({"error": message}))).into_response()
    }
}
