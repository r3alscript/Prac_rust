<<<<<<< HEAD
Write-Host "Clearing benchmark_results table..."

docker exec -i auction-postgres psql -U auction_user -d auction_db -c "TRUNCATE TABLE benchmark_results RESTART IDENTITY;"

=======
Write-Host "Clearing benchmark_results table..."

docker exec -i auction-postgres psql -U auction_user -d auction_db -c "TRUNCATE TABLE benchmark_results RESTART IDENTITY;"

>>>>>>> f63628c3a44df1d65bd9e805f0eef628cd04195e
Write-Host "benchmark_results table cleared."