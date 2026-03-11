use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BenchmarkSummaryDto {
    pub format: String,

    pub events_count: i64,

    pub avg_payload_size_bytes: f64,

    pub min_latency_ms: i64,
    pub avg_latency_ms: f64,
    pub max_latency_ms: i64,

    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,

    pub throughput_events_per_sec: f64,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DashboardResponse {
    pub json: BenchmarkSummaryDto,
    pub protobuf: BenchmarkSummaryDto,
    pub last_updated: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunBenchmarkResponse {
    pub success: bool,
    pub message: String,
}