
Write-Host "Clearing benchmark_results table..."

docker exec -i auction-postgres psql -U auction_user -d auction_db -c "TRUNCATE TABLE benchmark_results RESTART IDENTITY;"

Write-Host "benchmark_results table cleared."