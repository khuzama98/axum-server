use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    config::DatabasePool,
    error::{AppError, AppResult},
    models::{CreateUserRequest, UpdateUserRequest, User},
};

#[tracing::instrument(skip(pool))]
pub async fn create_user(
    State(pool): State<Arc<DatabasePool>>,
    Json(payload): Json<CreateUserRequest>,
) -> AppResult<(StatusCode, Json<User>)> {
    let user_id = Uuid::new_v4();
    let now = Utc::now();
    
    tracing::info!(
        user_id = %user_id,
        username = %payload.username,
        "Creating new user"
    );

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, username, email, first_name, last_name, bio, avatar_url, is_active, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&payload.bio)
    .bind(&payload.avatar_url)
    .bind(true) // is_active default to true
    .bind(now)
    .bind(now)
    .fetch_one(pool.as_ref())
    .await?;

    tracing::info!(
        user_id = %user.id,
        username = %user.username,
        "User created successfully"
    );

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user_by_id(
    State(pool): State<Arc<DatabasePool>>,
    Path(user_id): Path<Uuid>,
) -> AppResult<Json<User>> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE id = $1 AND is_active = true",
    )
    .bind(user_id)
    .fetch_one(pool.as_ref())
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => AppError::NotFound("User not found".to_string()),
        _ => AppError::from(err),
    })?;

    Ok(Json(user))
}

pub async fn get_all_users(
    State(pool): State<Arc<DatabasePool>>,
) -> AppResult<Json<Vec<User>>> {
    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE is_active = true ORDER BY created_at DESC",
    )
    .fetch_all(pool.as_ref())
    .await?;

    Ok(Json(users))
}

pub async fn update_user(
    State(pool): State<Arc<DatabasePool>>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> AppResult<Json<User>> {
    let now = Utc::now();

    // First check if user exists
    let _existing_user = sqlx::query!("SELECT id FROM users WHERE id = $1", user_id)
        .fetch_one(pool.as_ref())
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => AppError::NotFound("User not found".to_string()),
            _ => AppError::from(err),
        })?;

    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users 
        SET username = COALESCE($2, username),
            email = COALESCE($3, email),
            first_name = COALESCE($4, first_name),
            last_name = COALESCE($5, last_name),
            bio = COALESCE($6, bio),
            avatar_url = COALESCE($7, avatar_url),
            is_active = COALESCE($8, is_active),
            updated_at = $9
        WHERE id = $1
        RETURNING *
        "#,
    )
    .bind(user_id)
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&payload.bio)
    .bind(&payload.avatar_url)
    .bind(payload.is_active)
    .bind(now)
    .fetch_one(pool.as_ref())
    .await?;

    Ok(Json(user))
}

pub async fn delete_user(
    State(pool): State<Arc<DatabasePool>>,
    Path(user_id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let result = sqlx::query!("UPDATE users SET is_active = false WHERE id = $1", user_id)
        .execute(pool.as_ref())
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}