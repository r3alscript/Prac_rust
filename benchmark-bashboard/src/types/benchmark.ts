export type BenchmarkFormat = "json" | "protobuf";

export type BenchmarkSummary = {
    format: BenchmarkFormat;
    eventsCount: number;
    avgPayloadSizeBytes: number;
    minLatencyMs: number;
    avgLatencyMs: number;
    maxLatencyMs: number;
    p95LatencyMs: number;
    p99LatencyMs: number;
    throughputEventsPerSec: number;
};

export type BenchmarkDashboardData = {
    json: BenchmarkSummary;
    protobuf: BenchmarkSummary;
    lastUpdated: string;
};

export type RunBenchmarkResponse = {
    success: boolean;
    message: string;
};