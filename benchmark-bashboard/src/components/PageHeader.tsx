export default function PageHeader() {
    return (
        <header className="page-header">
            <p className="eyebrow">Event-Driven Benchmark</p>
            <h1>Порівняння JSON та Protobuf у системі онлайн-аукціону</h1>
            <p className="lead">
                Односторінкова панель для демонстрації генерації подій, вимірювання
                часу доставки та порівняння форматів передачі даних у Rust-based
                event-driven архітектурі.
            </p>
        </header>
    );
}