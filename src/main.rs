use std::sync::Arc;
use tracing_subscriber;

mod config;
mod error;
mod handlers;
mod models;
mod routes;

use config::{create_pool, run_migrations};
use routes::create_routes;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    
    // Initialize tracing with custom configuration
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    // Create database pool
    let pool = create_pool()
        .await
        .expect("Failed to create database pool");

    // Run migrations
    run_migrations(&pool)
        .await
        .expect("Failed to run migrations");

    // Create routes with shared state
    let app = create_routes().with_state(Arc::new(pool));

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server starting on http://0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}