import type {
    BenchmarkDashboardData,
    RunBenchmarkResponse
} from "../types/benchmark";

const API_BASE = "http://127.0.0.1:8081/api/benchmark";

async function safeJson<T>(response: Response): Promise<T> {
    if (!response.ok) {
        const text = await response.text();
        throw new Error(text || `HTTP ${response.status}`);
    }
    return response.json() as Promise<T>;
}


export async function getDashboardData(): Promise<BenchmarkDashboardData> {
    const response = await fetch(`${API_BASE}/dashboard`, {
        method: "GET"
    });

    return await safeJson<BenchmarkDashboardData>(response);
}

export async function runJsonBenchmark(): Promise<RunBenchmarkResponse> {
    const response = await fetch(`${API_BASE}/run/json`, {
        method: "POST"
    });

    return await safeJson<RunBenchmarkResponse>(response);
}

export async function runProtobufBenchmark(): Promise<RunBenchmarkResponse> {
    const response = await fetch(`${API_BASE}/run/protobuf`, {
        method: "POST"
    });

    return await safeJson<RunBenchmarkResponse>(response);
}

export async function runFullBenchmark(): Promise<RunBenchmarkResponse> {
    const response = await fetch(`${API_BASE}/run/full`, {
        method: "POST"
    });

    return await safeJson<RunBenchmarkResponse>(response);
}

/* 👇 ДОДАТИ ЦЕ */
export async function getBenchmarkProgress() {
    const response = await fetch(`${API_BASE}/progress`, {
        method: "GET"
    });

    return await safeJson<{
        step: number;
        status: string;
        jsonCount: number;
        protobufCount: number;
        isRunning: boolean;
    }>(response);
}