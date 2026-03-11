Write-Host "Exporting benchmark summary..."

docker exec -i auction-postgres psql -U auction_user -d auction_db -c "
SELECT
    format,
    COUNT(*) AS events_count,
    ROUND(AVG(payload_size_bytes)::numeric, 2) AS avg_payload_size_bytes,
    MIN(latency_ms) AS min_latency_ms,
    ROUND(AVG(latency_ms)::numeric, 2) AS avg_latency_ms,
    MAX(latency_ms) AS max_latency_ms,
    PERCENTILE_CONT(0.50) WITHIN GROUP (ORDER BY latency_ms) AS median_latency_ms,
    PERCENTILE_CONT(0.95) WITHIN GROUP (ORDER BY latency_ms) AS p95_latency_ms,
    PERCENTILE_CONT(0.99) WITHIN GROUP (ORDER BY latency_ms) AS p99_latency_ms
FROM benchmark_results
GROUP BY format
ORDER BY format;
"

Write-Host "Benchmark summary exported."