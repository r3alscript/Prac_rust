#[derive(Debug, Clone)]
pub struct BenchmarkWorkerConfig {
    pub rabbitmq_url: String,
    pub postgres_url: String,
    pub exchange: String,
    pub json_queue: String,
    pub protobuf_queue: String,
}

impl BenchmarkWorkerConfig {
    pub fn from_env() -> Self {
        Self {
            rabbitmq_url: std::env::var("RABBITMQ_URL")
                .unwrap_or_else(|_| "amqp://guest:guest@localhost:5673/%2f".to_string()),
            postgres_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5433/auction_db".to_string()),
            exchange: std::env::var("BENCHMARK_EXCHANGE")
                .unwrap_or_else(|_| "benchmark.exchange".to_string()),
            json_queue: std::env::var("BENCHMARK_JSON_QUEUE")
                .unwrap_or_else(|_| "benchmark.json.queue".to_string()),
            protobuf_queue: std::env::var("BENCHMARK_PROTOBUF_QUEUE")
                .unwrap_or_else(|_| "benchmark.protobuf.queue".to_string()),
        }
    }
}