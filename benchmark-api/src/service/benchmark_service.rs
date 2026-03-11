use std::sync::Arc;

use chrono::Utc;
use serde::Serialize;
use tokio::{
    process::Command,
    sync::RwLock,
    time::{sleep, Duration},
};

use shared::error::AppError;

use crate::{
    dto::dashboard_response::{BenchmarkSummaryDto, DashboardResponse, RunBenchmarkResponse},
    repository::benchmark_repository::{BenchmarkRepository, BenchmarkSummaryRow},
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkProgressState {
    pub step: u8,
    pub status: String,
    pub json_count: i64,
    pub protobuf_count: i64,
    pub is_running: bool,
}

#[derive(Clone)]
pub struct BenchmarkService {
    repository: BenchmarkRepository,
    progress: Arc<RwLock<BenchmarkProgressState>>,
}

impl BenchmarkService {
    pub fn new(repository: BenchmarkRepository) -> Self {
        Self {
            repository,
            progress: Arc::new(RwLock::new(BenchmarkProgressState {
                step: 0,
                status: "Очікування запуску".to_string(),
                json_count: 0,
                protobuf_count: 0,
                is_running: false,
            })),
        }
    }

    pub async fn get_dashboard(&self) -> Result<DashboardResponse, AppError> {
        let rows = self.repository.get_summary_rows().await?;
        let last_updated = self.repository.get_last_updated().await?;

        let json = rows
            .iter()
            .find(|row| row.format == "json")
            .map(map_row_to_dto)
            .unwrap_or_else(|| empty_summary("json"));

        let protobuf = rows
            .iter()
            .find(|row| row.format == "protobuf")
            .map(map_row_to_dto)
            .unwrap_or_else(|| empty_summary("protobuf"));

        Ok(DashboardResponse {
            json,
            protobuf,
            last_updated: last_updated.unwrap_or_else(Utc::now).to_rfc3339(),
        })
    }

    pub async fn get_progress(&self) -> BenchmarkProgressState {
        self.progress.read().await.clone()
    }

    pub async fn start_json_benchmark(&self) -> Result<RunBenchmarkResponse, AppError> {
        self.try_start(BenchmarkKind::Json).await
    }

    pub async fn start_protobuf_benchmark(&self) -> Result<RunBenchmarkResponse, AppError> {
        self.try_start(BenchmarkKind::Protobuf).await
    }

    pub async fn start_full_benchmark(&self) -> Result<RunBenchmarkResponse, AppError> {
        self.try_start(BenchmarkKind::Full).await
    }

    async fn try_start(&self, kind: BenchmarkKind) -> Result<RunBenchmarkResponse, AppError> {
        {
            let progress = self.progress.read().await;
            if progress.is_running {
                return Ok(RunBenchmarkResponse {
                    success: false,
                    message: "Benchmark is already running.".to_string(),
                });
            }
        }

        let service = self.clone();

        tokio::spawn(async move {
            let result = match kind {
                BenchmarkKind::Json => service.run_json_benchmark_internal().await,
                BenchmarkKind::Protobuf => service.run_protobuf_benchmark_internal().await,
                BenchmarkKind::Full => service.run_full_benchmark_internal().await,
            };

            if let Err(error) = result {
                eprintln!("Benchmark failed: {}", error);
                service
                    .set_progress(
                        0,
                        format!("Помилка benchmark: {}", error),
                        0,
                        0,
                        false,
                    )
                    .await;
            }
        });

        Ok(RunBenchmarkResponse {
            success: true,
            message: "Benchmark started successfully.".to_string(),
        })
    }

    async fn run_json_benchmark_internal(&self) -> Result<(), AppError> {
        self.repository.clear_results().await?;

        self.set_progress(1, "Запуск generator для JSON...", 0, 0, true)
            .await;

        self.run_generator("json").await?;

        self.set_progress(
            2,
            "JSON події відправлено. Очікування обробки consumer...",
            0,
            0,
            true,
        )
            .await;

        self.wait_for_expected_results(1000, 0).await?;

        self.set_progress(7, "JSON benchmark завершено.", 1000, 0, false)
            .await;

        Ok(())
    }

    async fn run_protobuf_benchmark_internal(&self) -> Result<(), AppError> {
        self.repository.clear_results().await?;

        self.set_progress(1, "Запуск generator для Protobuf...", 0, 0, true)
            .await;

        self.run_generator("protobuf").await?;

        self.set_progress(
            2,
            "Protobuf події відправлено. Очікування обробки consumer...",
            0,
            0,
            true,
        )
            .await;

        self.wait_for_expected_results(0, 1000).await?;

        self.set_progress(7, "Protobuf benchmark завершено.", 0, 1000, false)
            .await;

        Ok(())
    }

    async fn run_full_benchmark_internal(&self) -> Result<(), AppError> {
        self.repository.clear_results().await?;

        self.set_progress(1, "Паралельний запуск JSON і Protobuf generator...", 0, 0, true)
            .await;

        let json_fut = self.run_generator("json");
        let protobuf_fut = self.run_generator("protobuf");

        tokio::try_join!(json_fut, protobuf_fut)?;

        self.set_progress(
            2,
            "Генератори завершили відправку. Очікування запису всіх результатів у PostgreSQL...",
            0,
            0,
            true,
        )
            .await;

        self.wait_for_expected_results(1000, 1000).await?;

        self.set_progress(7, "Full benchmark завершено.", 1000, 1000, false)
            .await;

        Ok(())
    }

    async fn run_generator(&self, format: &str) -> Result<(), AppError> {
        let rabbitmq_url = std::env::var("BENCHMARK_RABBITMQ_URL")
            .unwrap_or_else(|_| "amqp://guest:guest@auction-rabbitmq:5672/%2f".to_string());

        let status = Command::new("cargo")
            .args([
                "run",
                "-p",
                "benchmark-generator",
                "--",
                "--format",
                format,
                "--count",
                "1000",
                "--rabbitmq-url",
                &rabbitmq_url,
            ])
            .status()
            .await
            .map_err(|e| AppError::Message(format!("Failed to start benchmark-generator: {}", e)))?;

        if !status.success() {
            return Err(AppError::Message(format!(
                "benchmark-generator failed for format {}",
                format
            )));
        }

        Ok(())
    }

    async fn wait_for_expected_results(
        &self,
        expected_json: i64,
        expected_protobuf: i64,
    ) -> Result<(), AppError> {
        let max_attempts = 240;
        let delay = Duration::from_millis(500);

        for _ in 0..max_attempts {
            let rows = self.repository.get_summary_rows().await?;
            let (json_count, protobuf_count) = extract_counts(&rows);

            let total_count = json_count + protobuf_count;
            let expected_total = expected_json + expected_protobuf;

            if total_count == 0 {
                self.set_progress(
                    3,
                    "Події вже в RabbitMQ. Очікування першого отримання consumer...",
                    json_count,
                    protobuf_count,
                    true,
                )
                    .await;
            } else if total_count < (expected_total.max(1) / 3).max(1) {
                self.set_progress(
                    4,
                    "Consumer отримує повідомлення і починає десеріалізацію...",
                    json_count,
                    protobuf_count,
                    true,
                )
                    .await;
            } else if total_count < ((expected_total.max(1) * 2) / 3).max(1) {
                self.set_progress(
                    5,
                    "Виконується обробка подій і обчислення latency...",
                    json_count,
                    protobuf_count,
                    true,
                )
                    .await;
            } else if total_count < expected_total {
                self.set_progress(
                    6,
                    "Метрики активно зберігаються в PostgreSQL...",
                    json_count,
                    protobuf_count,
                    true,
                )
                    .await;
            }

            let json_ready = json_count >= expected_json;
            let protobuf_ready = protobuf_count >= expected_protobuf;

            let done = match (expected_json > 0, expected_protobuf > 0) {
                (true, true) => json_ready && protobuf_ready,
                (true, false) => json_ready,
                (false, true) => protobuf_ready,
                (false, false) => true,
            };

            if done {
                return Ok(());
            }

            sleep(delay).await;
        }

        Err(AppError::Message(
            "Timed out while waiting for benchmark results".to_string(),
        ))
    }

    async fn set_progress(
        &self,
        step: u8,
        status: impl Into<String>,
        json_count: i64,
        protobuf_count: i64,
        is_running: bool,
    ) {
        let mut progress = self.progress.write().await;
        progress.step = step;
        progress.status = status.into();
        progress.json_count = json_count;
        progress.protobuf_count = protobuf_count;
        progress.is_running = is_running;
    }
}

#[derive(Clone, Copy)]
enum BenchmarkKind {
    Json,
    Protobuf,
    Full,
}

fn extract_counts(rows: &[BenchmarkSummaryRow]) -> (i64, i64) {
    let json_count = rows
        .iter()
        .find(|row| row.format == "json")
        .map(|row| row.events_count)
        .unwrap_or(0);

    let protobuf_count = rows
        .iter()
        .find(|row| row.format == "protobuf")
        .map(|row| row.events_count)
        .unwrap_or(0);

    (json_count, protobuf_count)
}

fn map_row_to_dto(row: &BenchmarkSummaryRow) -> BenchmarkSummaryDto {
    let duration_ms = (row.last_receive_time_ms - row.first_sent_time_ms).max(1) as f64;
    let throughput = row.events_count as f64 / (duration_ms / 1000.0);

    BenchmarkSummaryDto {
        format: row.format.clone(),
        events_count: row.events_count,
        avg_payload_size_bytes: row.avg_payload_size_bytes,
        min_latency_ms: row.min_latency_ms,
        avg_latency_ms: row.avg_latency_ms,
        max_latency_ms: row.max_latency_ms,
        p95_latency_ms: row.p95_latency_ms,
        p99_latency_ms: row.p99_latency_ms,
        throughput_events_per_sec: throughput,
    }
}

fn empty_summary(format: &str) -> BenchmarkSummaryDto {
    BenchmarkSummaryDto {
        format: format.to_string(),
        events_count: 0,
        avg_payload_size_bytes: 0.0,
        min_latency_ms: 0,
        avg_latency_ms: 0.0,
        max_latency_ms: 0,
        p95_latency_ms: 0.0,
        p99_latency_ms: 0.0,
        throughput_events_per_sec: 0.0,
    }
}