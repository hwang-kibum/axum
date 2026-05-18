use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    auth::AuthUser,
    error::AppError,
    models::post::{CreatePostRequest, Post, PostListResponse, PostQuery, UpdatePostRequest},
    services::post as post_service,
    AppState,
};

#[utoipa::path(
    get,
    path = "/posts",
    tag = "posts",
    params(PostQuery),
    responses(
        (status = 200, description = "게시글 목록 조회 성공", body = PostListResponse),
    )
)]
pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<PostQuery>,
) -> Result<Json<PostListResponse>, AppError> {
    let resp = post_service::list(&state.db, query).await?;
    Ok(Json(resp))
}

#[utoipa::path(
    get,
    path = "/posts/{id}",
    tag = "posts",
    params(("id" = i64, Path, description = "게시글 ID")),
    responses(
        (status = 200, description = "게시글 조회 성공", body = Post),
        (status = 404, description = "게시글 없음"),
    )
)]
pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Post>, AppError> {
    let post = post_service::get(&state.db, id).await?;
    Ok(Json(post))
}

#[utoipa::path(
    post,
    path = "/posts",
    tag = "posts",
    security(("bearer_auth" = [])),
    request_body = CreatePostRequest,
    responses(
        (status = 201, description = "게시글 작성 성공", body = Post),
        (status = 401, description = "인증 필요"),
    )
)]
pub async fn create(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(req): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<Post>), AppError> {
    let post = post_service::create(&state.db, req, claims.sub).await?;
    Ok((StatusCode::CREATED, Json(post)))
}

#[utoipa::path(
    put,
    path = "/posts/{id}",
    tag = "posts",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "게시글 ID")),
    request_body = UpdatePostRequest,
    responses(
        (status = 200, description = "게시글 수정 성공", body = Post),
        (status = 401, description = "인증 필요"),
        (status = 403, description = "작성자만 수정 가능"),
        (status = 404, description = "게시글 없음"),
    )
)]
pub async fn update(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<Post>, AppError> {
    let post = post_service::update(&state.db, id, req, claims.sub).await?;
    Ok(Json(post))
}

#[utoipa::path(
    delete,
    path = "/posts/{id}",
    tag = "posts",
    security(("bearer_auth" = [])),
    params(("id" = i64, Path, description = "게시글 ID")),
    responses(
        (status = 204, description = "게시글 삭제 성공"),
        (status = 401, description = "인증 필요"),
        (status = 403, description = "작성자만 삭제 가능"),
        (status = 404, description = "게시글 없음"),
    )
)]
pub async fn delete(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<i64>,
) -> Result<StatusCode, AppError> {
    post_service::delete(&state.db, id, claims.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}
