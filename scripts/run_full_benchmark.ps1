<<<<<<< HEAD
Write-Host "=== FULL BENCHMARK START ==="

powershell -ExecutionPolicy Bypass -File .\scripts\clear_benchmark_results.ps1

Write-Host ""
Write-Host "Run benchmark-worker in a separate terminal before continuing."
Write-Host ""

powershell -ExecutionPolicy Bypass -File .\scripts\run_json_benchmark.ps1
Start-Sleep -Seconds 3

powershell -ExecutionPolicy Bypass -File .\scripts\run_protobuf_benchmark.ps1
Start-Sleep -Seconds 3

powershell -ExecutionPolicy Bypass -File .\scripts\export_results.ps1
powershell -ExecutionPolicy Bypass -File .\scripts\export_raw_json_results.ps1
powershell -ExecutionPolicy Bypass -File .\scripts\export_raw_protobuf_results.ps1
powershell -ExecutionPolicy Bypass -File .\scripts\export_benchmark_table.ps1

=======
Write-Host "=== FULL BENCHMARK START ==="

powershell -ExecutionPolicy Bypass -File .\scripts\clear_benchmark_results.ps1

Write-Host ""
Write-Host "Run benchmark-worker in a separate terminal before continuing."
Write-Host ""

powershell -ExecutionPolicy Bypass -File .\scripts\run_json_benchmark.ps1
Start-Sleep -Seconds 3

powershell -ExecutionPolicy Bypass -File .\scripts\run_protobuf_benchmark.ps1
Start-Sleep -Seconds 3

powershell -ExecutionPolicy Bypass -File .\scripts\export_results.ps1
powershell -ExecutionPolicy Bypass -File .\scripts\export_raw_json_results.ps1
powershell -ExecutionPolicy Bypass -File .\scripts\export_raw_protobuf_results.ps1
powershell -ExecutionPolicy Bypass -File .\scripts\export_benchmark_table.ps1

>>>>>>> f63628c3a44df1d65bd9e805f0eef628cd04195e
Write-Host "=== FULL BENCHMARK END ==="