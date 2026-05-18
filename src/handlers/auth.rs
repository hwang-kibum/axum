use axum::{extract::State, Json};

use crate::{
    error::AppError,
    models::user::{AuthResponse, LoginRequest, RegisterRequest},
    services::auth as auth_service,
    AppState,
};

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "회원가입 성공", body = AuthResponse),
        (status = 400, description = "잘못된 요청 (사용자 이름 중복, 비밀번호 짧음 등)"),
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let resp = auth_service::register(&state.db, &state.jwt_secret, req).await?;
    Ok(Json(resp))
}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "로그인 성공", body = AuthResponse),
        (status = 401, description = "사용자 이름 또는 비밀번호 불일치"),
    )
)]
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let resp = auth_service::login(&state.db, &state.jwt_secret, req).await?;
    Ok(Json(resp))
}
