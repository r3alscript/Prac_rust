import type { BenchmarkDashboardData } from "../types/benchmark";

type Props = {
    data: BenchmarkDashboardData;
};

export default function SummaryCards({ data }: Props) {
    return (
        <section className="summary-grid">
            <div className="card summary-card">
                <h3>JSON</h3>
                <p><strong>Подій:</strong> {data.json.eventsCount}</p>
                <p><strong>Avg Payload:</strong> {data.json.avgPayloadSizeBytes.toFixed(2)} bytes</p>
                <p><strong>Avg Latency:</strong> {data.json.avgLatencyMs.toFixed(2)} ms</p>
                <p><strong>Throughput:</strong> {data.json.throughputEventsPerSec.toFixed(2)} events/s</p>
            </div>

            <div className="card summary-card">
                <h3>Protobuf</h3>
                <p><strong>Подій:</strong> {data.protobuf.eventsCount}</p>
                <p><strong>Avg Payload:</strong> {data.protobuf.avgPayloadSizeBytes.toFixed(2)} bytes</p>
                <p><strong>Avg Latency:</strong> {data.protobuf.avgLatencyMs.toFixed(2)} ms</p>
                <p><strong>Throughput:</strong> {data.protobuf.throughputEventsPerSec.toFixed(2)} events/s</p>
            </div>
        </section>
    );
}