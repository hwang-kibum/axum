mod auth;
mod config;
mod error;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

use std::sync::Arc;

use sqlx::MySqlPool;
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<MySqlPool>,
    pub jwt_secret: String,
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

    // 정적 파일 (프론트엔드) — 라우트 매칭 실패 시 fallback
    let app = routes::app_router(state)
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    info!("서버 시작:    http://{}", config.bind_addr);
    info!("Swagger UI:  http://{}/swagger-ui/", config.bind_addr);
    axum::serve(listener, app).await?;

    Ok(())
}
