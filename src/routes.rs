use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::handlers;

pub fn create_router(pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/api/sync/push", axum::routing::post(handlers::push_diagram))
        .route("/api/sync/pull/:id", get(handlers::pull_diagram))
        .with_state(pool)
}

