mod auth;
mod config;
mod error;
mod handlers;
mod models;
mod repositories;
mod services;

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, routing::{get, post}, Router};
use sqlx::MySqlPool;
use tower_http::services::ServeDir;
use tracing::info;
use utoipa::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa_swagger_ui::SwaggerUi;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<MySqlPool>,
    pub jwt_secret: String,
}

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
struct ApiDoc;

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = config::Config::from_env();

    let pool = MySqlPool::connect(&config.database_url).await?;
    info!("데이터베이스 연결 완료");

    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("마이그레이션 완료");

    let state = AppState {
        db: Arc::new(pool),
        jwt_secret: config.jwt_secret,
    };

    let app = Router::new()
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // 헬스체크
        .route("/health", get(health_check))
        // 인증
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        // 게시판
        .route("/posts", get(handlers::post::list).post(handlers::post::create))
        .route(
            "/posts/{id}",
            get(handlers::post::get)
                .put(handlers::post::update)
                .delete(handlers::post::delete),
        )
        .with_state(state)
        // 정적 파일 (프론트엔드) — 라우트 매칭 실패 시 fallback
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    info!("서버 시작:    http://{}", config.bind_addr);
    info!("Swagger UI:  http://{}/swagger-ui/", config.bind_addr);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check(State(state): State<AppState>) -> StatusCode {
    match state.db.acquire().await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::SERVICE_UNAVAILABLE,
    }
}
