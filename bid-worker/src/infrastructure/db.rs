<<<<<<< HEAD
use shared::error::AppError;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub type PgPool = Pool<Postgres>;

pub async fn create_db_pool(database_url: &str) -> Result<PgPool, AppError> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
=======
use shared::error::AppError;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub type PgPool = Pool<Postgres>;

pub async fn create_db_pool(database_url: &str) -> Result<PgPool, AppError> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .map_err(|e| AppError::Database(e.to_string()))
>>>>>>> f63628c3a44df1d65bd9e805f0eef628cd04195e
}