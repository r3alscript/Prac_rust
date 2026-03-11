Write-Host "Starting JSON benchmark..."

cargo run -p benchmark-generator -- `
  --format json `
  --count 1000 `
  --rabbitmq-url amqp://guest:guest@localhost:5673/%2f `
  --exchange benchmark.exchange

Write-Host "JSON benchmark completed."