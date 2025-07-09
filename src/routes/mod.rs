pub mod user;

use axum::{routing::get, Router};
use std::sync::Arc;

use crate::config::DatabasePool;
use user::user_routes;

async fn health_check() -> &'static str {
    "OK"
}

async fn root() -> &'static str {
    "Hello, World!"
}

pub fn create_routes() -> Router<Arc<DatabasePool>> {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .merge(user_routes())
}