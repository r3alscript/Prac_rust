<<<<<<< HEAD
Write-Host "Exporting raw Protobuf benchmark results..."

docker exec -i auction-postgres psql -U auction_user -d auction_db -c "\copy (
    SELECT event_id, format, payload_size_bytes, sent_time_ms, receive_time_ms, latency_ms
    FROM benchmark_results
    WHERE format = 'protobuf'
    ORDER BY id
) TO STDOUT WITH CSV HEADER" > reports\benchmark-results\protobuf_results.csv

=======
Write-Host "Exporting raw Protobuf benchmark results..."

docker exec -i auction-postgres psql -U auction_user -d auction_db -c "\copy (
    SELECT event_id, format, payload_size_bytes, sent_time_ms, receive_time_ms, latency_ms
    FROM benchmark_results
    WHERE format = 'protobuf'
    ORDER BY id
) TO STDOUT WITH CSV HEADER" > reports\benchmark-results\protobuf_results.csv

>>>>>>> f63628c3a44df1d65bd9e805f0eef628cd04195e
Write-Host "Protobuf results exported to reports\benchmark-results\protobuf_results.csv"