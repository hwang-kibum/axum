pub struct Config {
    pub database_url: String,
    pub bind_addr: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL 환경변수 필요"),
            bind_addr: std::env::var("BIND_ADDR")
                .unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .expect("JWT_SECRET 환경변수 필요"),
        }
    }
}
