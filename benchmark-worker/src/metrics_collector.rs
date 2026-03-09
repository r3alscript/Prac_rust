use chrono::Utc;
use shared::{
    events::BidPlacedEvent,
    metrics::benchmark_result::BenchmarkResult,
    serialization::format::MessageFormat,
};

pub fn collect_metrics(
    event: &BidPlacedEvent,
    format: MessageFormat,
    payload_size_bytes: usize,
) -> BenchmarkResult {
    let receive_time_ms = Utc::now().timestamp_millis();
    let latency_ms = receive_time_ms - event.sent_time_ms;

    BenchmarkResult {
        event_id: event.event_id.clone(),
        format: format.as_str().to_string(),
        payload_size_bytes,
        sent_time_ms: event.sent_time_ms,
        receive_time_ms,
        latency_ms,
    }
}