Write-Host "Exporting benchmark table..."

$result = docker exec -i auction-postgres psql -U auction_user -d auction_db -t -A -F "," -c "
SELECT
    format,
    COUNT(*) AS events_count,
    ROUND(AVG(payload_size_bytes)::numeric, 2) AS avg_payload_size_bytes,
    MIN(latency_ms) AS min_latency_ms,
    ROUND(AVG(latency_ms)::numeric, 2) AS avg_latency_ms,
    MAX(latency_ms) AS max_latency_ms,
    PERCENTILE_CONT(0.95) WITHIN GROUP (ORDER BY latency_ms) AS p95_latency_ms,
    PERCENTILE_CONT(0.99) WITHIN GROUP (ORDER BY latency_ms) AS p99_latency_ms
FROM benchmark_results
GROUP BY format
ORDER BY format;
"

$lines = @()
$lines += "| Format | Events Count | Avg Payload Size (bytes) | Min Latency (ms) | Avg Latency (ms) | Max Latency (ms) | P95 (ms) | P99 (ms) |"
$lines += "|--------|--------------|---------------------------|------------------|------------------|------------------|----------|----------|"

foreach ($row in $result) {
    if ($row.Trim() -ne "") {
        $cols = $row.Split(",")
        $lines += "| $($cols[0]) | $($cols[1]) | $($cols[2]) | $($cols[3]) | $($cols[4]) | $($cols[5]) | $($cols[6]) | $($cols[7]) |"
    }
}

$dir = "reports\benchmark-results"
if (!(Test-Path $dir)) {
    New-Item -ItemType Directory -Path $dir | Out-Null
}

$lines | Set-Content "$dir\benchmark_table.md"

Write-Host "Benchmark table exported to reports\benchmark-results\benchmark_table.md"