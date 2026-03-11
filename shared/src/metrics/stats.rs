use super::benchmark_result::BenchmarkResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSummary {
    pub format: String,
    pub events_count: usize,
    pub avg_payload_size_bytes: f64,
    pub min_latency_ms: i64,
    pub avg_latency_ms: f64,
    pub max_latency_ms: i64,
    pub median_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
}

impl BenchmarkSummary {
    pub fn empty(format: impl Into<String>) -> Self {
        Self {
            format: format.into(),
            events_count: 0,
            avg_payload_size_bytes: 0.0,
            min_latency_ms: 0,
            avg_latency_ms: 0.0,
            max_latency_ms: 0,
            median_latency_ms: 0.0,
            p95_latency_ms: 0.0,
            p99_latency_ms: 0.0,
        }
    }
}

pub fn calculate_summary(
    format: impl Into<String>,
    results: &[BenchmarkResult],
) -> BenchmarkSummary {
    let format = format.into();

    if results.is_empty() {
        return BenchmarkSummary::empty(format);
    }

    let mut latencies: Vec<i64> = results.iter().map(|r| r.latency_ms).collect();
    latencies.sort_unstable();

    let payload_sum: usize = results.iter().map(|r| r.payload_size_bytes).sum();
    let latency_sum: i64 = latencies.iter().sum();

    let min_latency = *latencies.first().unwrap_or(&0);
    let max_latency = *latencies.last().unwrap_or(&0);

    BenchmarkSummary {
        format,
        events_count: results.len(),
        avg_payload_size_bytes: payload_sum as f64 / results.len() as f64,
        min_latency_ms: min_latency,
        avg_latency_ms: latency_sum as f64 / results.len() as f64,
        max_latency_ms: max_latency,
        median_latency_ms: percentile(&latencies, 50.0),
        p95_latency_ms: percentile(&latencies, 95.0),
        p99_latency_ms: percentile(&latencies, 99.0),
    }
}

fn percentile(sorted: &[i64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }

    if sorted.len() == 1 {
        return sorted[0] as f64;
    }

    let rank = (p / 100.0) * (sorted.len() - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;

    if lower == upper {
        sorted[lower] as f64
    } else {
        let weight = rank - lower as f64;
        sorted[lower] as f64 * (1.0 - weight) + sorted[upper] as f64 * weight
    }
}