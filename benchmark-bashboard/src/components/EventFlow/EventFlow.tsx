type EventFlowProps = {
    activeStep: number;
    isRunning: boolean;
};

const steps = [
    "Generator.rs",
    "Publisher.rs",
    "Message Broker",
    "Consumer.rs",
    "Process_bid_event.rs",
    "Metrics_collector.rs",
    "Repository.rs"
];

export default function EventFlow({ activeStep, isRunning }: EventFlowProps) {
    return (
        <section className="event-flow-card">
            <div className="event-flow-header">
                <div>
                    <h2>Шлях передачі події</h2>

                </div>

                <div className={`flow-badge ${isRunning ? "running" : "idle"}`}>
                    {isRunning ? "Виконується benchmark" : "Очікування запуску"}
                </div>
            </div>

            <div className="event-flow-line">
                {steps.map((step, index) => {
                    const stepNumber = index + 1;
                    const isActive = isRunning && activeStep === stepNumber;
                    const isCompleted = isRunning && stepNumber < activeStep;

                    return (
                        <div className="event-flow-item-wrapper" key={step}>
                            <div
                                className={[
                                    "event-flow-item",
                                    isActive ? "active" : "",
                                    isCompleted ? "completed" : "",
                                ].join(" ")}
                            >
                                <div className="event-flow-item-top">
                                    <span className="event-step-number">{stepNumber}</span>
                                    <span className="event-step-status">
                    {isActive
                        ? "Виконується"
                        : isCompleted
                            ? "Завершено"
                            : "Очікує"}
                  </span>
                                </div>

                                <div className="event-step-title">{step}</div>
                            </div>

                            {index < steps.length - 1 && <div className="event-flow-connector" />}
                        </div>
                    );
                })}
            </div>
        </section>
    );
}