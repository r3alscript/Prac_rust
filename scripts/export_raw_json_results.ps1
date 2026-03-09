Write-Host "Exporting raw JSON benchmark results..."

docker exec -i auction-postgres psql -U auction_user -d auction_db -c "\copy (
    SELECT event_id, format, payload_size_bytes, sent_time_ms, receive_time_ms, latency_ms
    FROM benchmark_results
    WHERE format = 'json'
    ORDER BY id
) TO STDOUT WITH CSV HEADER" > reports\benchmark-results\json_results.csv

Write-Host "JSON results exported to reports\benchmark-results\json_results.csv"