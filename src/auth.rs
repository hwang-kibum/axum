use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{error::AppError, models::user::Claims, AppState};

/// Authorization: Bearer <token> 헤더에서 JWT를 검증하고 Claims를 추출하는 추출기
pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let auth = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("인증 토큰이 없습니다".to_string()))?;

        let token = auth
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Bearer 토큰 형식이 아닙니다".to_string()))?;

        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(AuthUser(data.claims))
    }
}
