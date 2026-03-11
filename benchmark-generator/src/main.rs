mod benchmark;
mod cli;
mod generator;
mod publisher;

use benchmark::run_benchmark;
use cli::BenchmarkCli;

#[tokio::main]
async fn main() {
    let cli = BenchmarkCli::parse_args();

    if let Err(err) = run_benchmark(cli).await {
        eprintln!("Benchmark generator failed: {}", err);
        std::process::exit(1);
    }
}