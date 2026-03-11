
use crate::{cli::BenchmarkCli, generator::generate_bid_event, publisher::BenchmarkPublisher};
use shared::error::AppError;
use tokio::time::{sleep, Duration};

pub async fn run_benchmark(cli: BenchmarkCli) -> Result<(), AppError> {
    let format = cli
        .message_format()
        .map_err(AppError::InvalidFormat)?;

    let publisher = BenchmarkPublisher::new(&cli.rabbitmq_url, &cli.exchange).await?;

    println!("Starting benchmark generator...");
    println!("RabbitMQ URL: {}", cli.rabbitmq_url);
    println!("Exchange: {}", cli.exchange);
    println!("Format: {}", format.as_str());
    println!("Events count: {}", cli.count);
    println!("Delay per message: {} ms", cli.delay_ms);
    println!("Routing key: {}", format.routing_key());
    println!();

    let mut total_bytes = 0usize;

    for i in 0..cli.count {
        let event = generate_bid_event(i);
        let payload_size = publisher.publish(&event, format).await?;
        total_bytes += payload_size;

        println!(
            "[{}/{}] event_id={} payload_size={} bytes sent_time_ms={}",
            i + 1,
            cli.count,
            event.event_id,
            payload_size,
            event.sent_time_ms
        );

        if cli.delay_ms > 0 {
            sleep(Duration::from_millis(cli.delay_ms)).await;
        }
    }

    let avg_size = if cli.count > 0 {
        total_bytes as f64 / cli.count as f64
    } else {
        0.0
    };

    println!();
    println!("Benchmark sending completed.");
    println!("Total events sent: {}", cli.count);
    println!("Total payload bytes: {}", total_bytes);
    println!("Average payload size: {:.2} bytes", avg_size);

    Ok(())
}