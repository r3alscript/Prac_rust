use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub event_id: String,
    pub format: String,
    pub payload_size_bytes: usize,
    pub sent_time_ms: i64,
    pub receive_time_ms: i64,
    pub latency_ms: i64,
}