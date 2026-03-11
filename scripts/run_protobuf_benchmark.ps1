
Write-Host "Starting Protobuf benchmark..."

cargo run -p benchmark-generator -- `
  --format protobuf `
  --count 1000 `
  --rabbitmq-url amqp://guest:guest@localhost:5673/%2f `
  --exchange benchmark.exchange

Write-Host "Protobuf benchmark completed."