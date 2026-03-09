mod config;
mod consumer;
mod metrics_collector;
mod repository;

use config::BenchmarkWorkerConfig;
use consumer::run_consumer;

#[tokio::main]
async fn main() {
    let config = BenchmarkWorkerConfig::from_env();

    if let Err(err) = run_consumer(config).await {
        eprintln!("Benchmark worker failed: {}", err);
        std::process::exit(1);
    }
}