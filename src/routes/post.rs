use axum::{Router, routing::get};

use crate::{handlers, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/posts", get(handlers::post::list).post(handlers::post::create))
        .route(
            "/posts/{id}",
            get(handlers::post::get)
                .put(handlers::post::update)
                .delete(handlers::post::delete),
        )
}
