<<<<<<< HEAD
Write-Host "Starting JSON benchmark..."

cargo run -p benchmark-generator -- `
  --format json `
  --count 1000 `
  --rabbitmq-url amqp://guest:guest@localhost:5673/%2f `
  --exchange benchmark.exchange

=======
Write-Host "Starting JSON benchmark..."

cargo run -p benchmark-generator -- `
  --format json `
  --count 1000 `
  --rabbitmq-url amqp://guest:guest@localhost:5673/%2f `
  --exchange benchmark.exchange

>>>>>>> f63628c3a44df1d65bd9e805f0eef628cd04195e
Write-Host "JSON benchmark completed."