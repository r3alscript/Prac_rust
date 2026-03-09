use shared::{error::AppError, metrics::benchmark_result::BenchmarkResult};
use sqlx::PgPool;

pub struct BenchmarkRepository {
    pool: PgPool,
}

impl BenchmarkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_result(&self, result: &BenchmarkResult) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO benchmark_results
            (event_id, format, payload_size_bytes, sent_time_ms, receive_time_ms, latency_ms)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
            .bind(&result.event_id)
            .bind(&result.format)
            .bind(result.payload_size_bytes as i32)
            .bind(result.sent_time_ms)
            .bind(result.receive_time_ms)
            .bind(result.latency_ms)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}