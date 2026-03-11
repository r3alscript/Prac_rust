import { useEffect, useRef, useState } from "react";
import type { BenchmarkDashboardData } from "./types/benchmark";
import {
    getDashboardData,
    getBenchmarkProgress,
    runFullBenchmark,
    runJsonBenchmark,
    runProtobufBenchmark
} from "./api/benchmarkApi";

import EventFlow from "./components/EventFlow/EventFlow";
import SummaryCards from "./components/SummaryCards";
import MetricsTable from "./components/MetricsTable";
import ControlsPanel from "./components/ControlsPanel";

type BenchmarkType = "full" | "json" | "protobuf";

export default function App() {
    const [data, setData] = useState<BenchmarkDashboardData | null>(null);
    const [loading, setLoading] = useState(true);
    const [busy, setBusy] = useState(false);
    const [status, setStatus] = useState("Готово до запуску benchmark.");
    const [activeStep, setActiveStep] = useState(0);

    const progressIntervalRef = useRef<number | null>(null);
    const dashboardReloadedRef = useRef(false);

    const loadData = async () => {
        const result = await getDashboardData();
        setData(result);
        return result;
    };

    const stopProgressPolling = () => {
        if (progressIntervalRef.current !== null) {
            window.clearInterval(progressIntervalRef.current);
            progressIntervalRef.current = null;
        }
    };

    const startProgressPolling = () => {
        stopProgressPolling();

        progressIntervalRef.current = window.setInterval(async () => {
            try {
                const progress = await getBenchmarkProgress();

                setActiveStep(progress.step ?? 0);
                setStatus(progress.status ?? "Виконується benchmark...");
                setBusy(Boolean(progress.isRunning));

                if (!progress.isRunning) {
                    stopProgressPolling();

                    if (!dashboardReloadedRef.current) {
                        dashboardReloadedRef.current = true;
                        await loadData();
                    }
                }
            } catch (error) {
                console.error("Failed to fetch progress:", error);
            }
        }, 500);
    };

    useEffect(() => {
        const init = async () => {
            try {
                setLoading(true);
                await loadData();

                const progress = await getBenchmarkProgress();
                setActiveStep(progress.step ?? 0);
                setStatus(progress.status ?? "Готово до запуску benchmark.");
                setBusy(Boolean(progress.isRunning));

                if (progress.isRunning) {
                    dashboardReloadedRef.current = false;
                    startProgressPolling();
                }
            } catch (error) {
                console.error(error);
                setStatus("Не вдалося завантажити dashboard-дані.");
            } finally {
                setLoading(false);
            }
        };

        void init();

        return () => {
            stopProgressPolling();
        };
    }, []);

    const handleRun = async (type: BenchmarkType) => {
        try {
            dashboardReloadedRef.current = false;
            setBusy(true);
            setActiveStep(1);

            startProgressPolling();

            if (type === "full") {
                setStatus("Виконується повний benchmark...");
                await runFullBenchmark();
            } else if (type === "json") {
                setStatus("Виконується benchmark у форматі JSON...");
                await runJsonBenchmark();
            } else {
                setStatus("Виконується benchmark у форматі Protobuf...");
                await runProtobufBenchmark();
            }
        } catch (error) {
            console.error(error);
            stopProgressPolling();
            setBusy(false);
            setStatus("Під час виконання benchmark виникла помилка.");
        }
    };

    if (loading) {
        return (
            <main className="page-shell">
                <div className="loading-box">Завантаження dashboard...</div>
            </main>
        );
    }

    return (
        <main className="page-shell">
            <div className="dashboard-top">
                <ControlsPanel
                    busy={busy}
                    status={status}
                    onRunFull={() => handleRun("full")}
                    onRunJson={() => handleRun("json")}
                    onRunProtobuf={() => handleRun("protobuf")}
                    onRefresh={() => {
                        void loadData();
                    }}
                />

                <EventFlow activeStep={activeStep} isRunning={busy} />
            </div>

            {data && (
                <>
                    <SummaryCards data={data} />

                    <div className="metrics-row">
                        <MetricsTable title="JSON Metrics" data={data.json} />
                        <MetricsTable title="Protobuf Metrics" data={data.protobuf} />
                    </div>

                    <div className="last-updated-wrap">
                        <div className="last-updated-chip">
                            Останнє оновлення: {new Date(data.lastUpdated).toLocaleString()}
                        </div>
                    </div>
                </>
            )}
        </main>
    );
}