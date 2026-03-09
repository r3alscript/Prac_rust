Write-Host "Starting benchmark worker..."

$env:RABBITMQ_URL = "amqp://guest:guest@localhost:5673/%2f"
$env:DATABASE_URL = "postgres://auction_user:auction_password@localhost:5433/auction_db"
$env:BENCHMARK_EXCHANGE = "benchmark.exchange"
$env:BENCHMARK_JSON_QUEUE = "benchmark.json.queue"
$env:BENCHMARK_PROTOBUF_QUEUE = "benchmark.protobuf.queue"

cargo run -p benchmark-worker