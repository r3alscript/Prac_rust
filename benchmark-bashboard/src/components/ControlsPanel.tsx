type Props = {
    busy: boolean;
    status: string;
    onRunJson: () => void;
    onRunProtobuf: () => void;
    onRunFull: () => void;
    onRefresh: () => void;
};

export default function ControlsPanel({
                                          busy,
                                          status,
                                          onRunJson,
                                          onRunProtobuf,
                                          onRunFull,
                                          onRefresh
                                      }: Props) {
    return (
        <section className="card">
            <h2>Керування benchmark</h2>

            <div className="controls">
                <button onClick={onRunJson} disabled={busy}>Run JSON Benchmark</button>
                <button onClick={onRunProtobuf} disabled={busy}>Run Protobuf Benchmark</button>
                <button onClick={onRunFull} disabled={busy} className="primary">
                    Run Full Benchmark
                </button>
                <button onClick={onRefresh} disabled={busy}>Refresh</button>
            </div>

            <p className="status-line">
                <strong>Статус:</strong> {status}
            </p>
        </section>
    );
}