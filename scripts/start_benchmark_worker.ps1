<<<<<<< HEAD
Write-Host "Starting benchmark worker..."

$env:RABBITMQ_URL = "amqp://guest:guest@localhost:5673/%2f"
$env:DATABASE_URL = "postgres://auction_user:auction_password@localhost:5433/auction_db"
$env:BENCHMARK_EXCHANGE = "benchmark.exchange"
$env:BENCHMARK_JSON_QUEUE = "benchmark.json.queue"
$env:BENCHMARK_PROTOBUF_QUEUE = "benchmark.protobuf.queue"

=======
Write-Host "Starting benchmark worker..."

$env:RABBITMQ_URL = "amqp://guest:guest@localhost:5673/%2f"
$env:DATABASE_URL = "postgres://auction_user:auction_password@localhost:5433/auction_db"
$env:BENCHMARK_EXCHANGE = "benchmark.exchange"
$env:BENCHMARK_JSON_QUEUE = "benchmark.json.queue"
$env:BENCHMARK_PROTOBUF_QUEUE = "benchmark.protobuf.queue"

>>>>>>> f63628c3a44df1d65bd9e805f0eef628cd04195e
cargo run -p benchmark-worker