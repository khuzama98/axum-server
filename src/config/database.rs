use sqlx::{PgPool, Pool, Postgres};
use std::env;

pub type DatabasePool = Pool<Postgres>;

pub async fn create_pool() -> Result<DatabasePool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/axum_users".to_string());
    
    tracing::info!("Connecting to database: {}", 
        // Hide password in logs for security
        database_url.replace(":postgres@", ":****@")
    );
    
    let pool = PgPool::connect(&database_url).await?;
    tracing::info!("Database connection established");
    
    Ok(pool)
}

pub async fn run_migrations(pool: &DatabasePool) -> Result<(), sqlx::migrate::MigrateError> {
    tracing::info!("Running database migrations...");
    let result = sqlx::migrate!("./migrations").run(pool).await;
    
    match &result {
        Ok(_) => tracing::info!("Database migrations completed successfully"),
        Err(e) => tracing::error!("Database migration failed: {}", e),
    }
    
    result
}