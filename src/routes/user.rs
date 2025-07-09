use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;

use crate::{
    config::DatabasePool,
    handlers::{create_user, get_user_by_id, get_all_users, update_user, delete_user},
};

pub fn user_routes() -> Router<Arc<DatabasePool>> {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_all_users))
        .route("/users/{id}", get(get_user_by_id))
        .route("/users/{id}", put(update_user))
        .route("/users/{id}", delete(delete_user))
}