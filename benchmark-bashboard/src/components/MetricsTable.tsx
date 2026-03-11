import type { BenchmarkSummary } from "../types/benchmark";

type Props = {
    title: string;
    data: BenchmarkSummary;
};

export default function MetricsTable({ title, data }: Props) {
    return (
        <section className="card">
            <h2>{title}</h2>

            <table className="metrics-table">
                <thead>
                <tr>
                    <th>Метрика</th>
                    <th>Значення</th>
                </tr>
                </thead>
                <tbody>
                <tr>
                    <td>Формат</td>
                    <td>{data.format}</td>
                </tr>
                <tr>
                    <td>Кількість подій</td>
                    <td>{data.eventsCount}</td>
                </tr>
                <tr>
                    <td>Середній розмір повідомлення</td>
                    <td>{data.avgPayloadSizeBytes.toFixed(2)} bytes</td>
                </tr>
                <tr>
                    <td>Мінімальна latency</td>
                    <td>{data.minLatencyMs} ms</td>
                </tr>
                <tr>
                    <td>Середня latency</td>
                    <td>{data.avgLatencyMs.toFixed(2)} ms</td>
                </tr>
                <tr>
                    <td>Максимальна latency</td>
                    <td>{data.maxLatencyMs} ms</td>
                </tr>
                <tr>
                    <td>P95 latency</td>
                    <td>{data.p95LatencyMs.toFixed(2)} ms</td>
                </tr>
                <tr>
                    <td>P99 latency</td>
                    <td>{data.p99LatencyMs.toFixed(2)} ms</td>
                </tr>
                <tr>
                    <td>Throughput</td>
                    <td>{data.throughputEventsPerSec.toFixed(2)} events/s</td>
                </tr>
                </tbody>
            </table>
        </section>
    );
}