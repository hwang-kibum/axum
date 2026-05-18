pub mod auth;
pub mod post;

use axum::{Router, extract::State, http::StatusCode, routing::get};
use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;

use crate::{handlers, models, AppState};

#[derive(OpenApi)]
#[openapi(
    info(title = "게시판 API", version = "1.0.0", description = "Axum 기반 게시판 REST API"),
    paths(
        handlers::auth::register,
        handlers::auth::login,
        handlers::post::list,
        handlers::post::get,
        handlers::post::create,
        handlers::post::update,
        handlers::post::delete,
    ),
    components(schemas(
        models::user::RegisterRequest,
        models::user::LoginRequest,
        models::user::AuthResponse,
        models::post::Post,
        models::post::CreatePostRequest,
        models::post::UpdatePostRequest,
        models::post::PostListResponse,
    )),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "회원가입 / 로그인"),
        (name = "posts", description = "게시판 CRUD"),
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

pub fn app_router(state: AppState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_check))
        .merge(auth::router())
        .merge(post::router())
        .with_state(state)
}

async fn health_check(State(state): State<AppState>) -> StatusCode {
    match state.db.acquire().await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}
