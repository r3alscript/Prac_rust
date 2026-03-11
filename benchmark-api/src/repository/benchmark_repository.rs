use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};

use shared::error::AppError;

#[derive(Debug, Clone)]
pub struct BenchmarkSummaryRow {
    pub format: String,
    pub events_count: i64,
    pub avg_payload_size_bytes: f64,
    pub min_latency_ms: i64,
    pub avg_latency_ms: f64,
    pub max_latency_ms: i64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub first_sent_time_ms: i64,
    pub last_receive_time_ms: i64,
}

#[derive(Clone)]
pub struct BenchmarkRepository {
    pool: PgPool,
}

impl BenchmarkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_summary_rows(&self) -> Result<Vec<BenchmarkSummaryRow>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT
                format,
                COUNT(*)::BIGINT AS events_count,
                AVG(payload_size_bytes)::FLOAT8 AS avg_payload_size_bytes,
                MIN(latency_ms)::BIGINT AS min_latency_ms,
                AVG(latency_ms)::FLOAT8 AS avg_latency_ms,
                MAX(latency_ms)::BIGINT AS max_latency_ms,
                PERCENTILE_CONT(0.95) WITHIN GROUP (ORDER BY latency_ms)::FLOAT8 AS p95_latency_ms,
                PERCENTILE_CONT(0.99) WITHIN GROUP (ORDER BY latency_ms)::FLOAT8 AS p99_latency_ms,
                MIN(sent_time_ms)::BIGINT AS first_sent_time_ms,
                MAX(receive_time_ms)::BIGINT AS last_receive_time_ms
            FROM benchmark_results
            GROUP BY format
            ORDER BY format
            "#,
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let result = rows
            .into_iter()
            .map(|row| BenchmarkSummaryRow {
                format: row.get::<String, _>("format"),
                events_count: row.get::<i64, _>("events_count"),
                avg_payload_size_bytes: row.get::<f64, _>("avg_payload_size_bytes"),
                min_latency_ms: row.get::<i64, _>("min_latency_ms"),
                avg_latency_ms: row.get::<f64, _>("avg_latency_ms"),
                max_latency_ms: row.get::<i64, _>("max_latency_ms"),
                p95_latency_ms: row.get::<f64, _>("p95_latency_ms"),
                p99_latency_ms: row.get::<f64, _>("p99_latency_ms"),
                first_sent_time_ms: row.get::<i64, _>("first_sent_time_ms"),
                last_receive_time_ms: row.get::<i64, _>("last_receive_time_ms"),
            })
            .collect();

        Ok(result)
    }

    pub async fn get_last_updated(&self) -> Result<Option<DateTime<Utc>>, AppError> {
        let row = sqlx::query(
            r#"
            SELECT MAX(created_at) AS last_updated
            FROM benchmark_results
            "#,
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let last_updated: Option<chrono::NaiveDateTime> = row.try_get("last_updated").ok();

        Ok(last_updated.map(|value| DateTime::<Utc>::from_naive_utc_and_offset(value, Utc)))
    }

    pub async fn clear_results(&self) -> Result<(), AppError> {
        sqlx::query("TRUNCATE TABLE benchmark_results RESTART IDENTITY")
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}