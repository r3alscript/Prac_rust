
use std::sync::Arc;

use futures_util::StreamExt;
use lapin::{
    options::{
        BasicAckOptions, BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions,
        QueueDeclareOptions,
    },
    types::FieldTable,
    Channel, Connection, ConnectionProperties, ExchangeKind,
};
use shared::{
    error::AppError,
    serialization::{deserialize_event, format::MessageFormat},
};
use sqlx::PgPool;

use crate::{
    config::BenchmarkWorkerConfig,
    metrics_collector::collect_metrics,
    repository::BenchmarkRepository,
};

pub async fn run_consumer(config: BenchmarkWorkerConfig) -> Result<(), AppError> {
    let pool = PgPool::connect(&config.postgres_url)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    let repository = Arc::new(BenchmarkRepository::new(pool));

    let connection = Connection::connect(&config.rabbitmq_url, ConnectionProperties::default())
        .await
        .map_err(|e| AppError::RabbitMq(e.to_string()))?;

    let channel = connection
        .create_channel()
        .await
        .map_err(|e| AppError::RabbitMq(e.to_string()))?;

    setup_benchmark_topology(&channel, &config).await?;

    consume_queue(
        &channel,
        &config.json_queue,
        MessageFormat::Json,
        repository.clone(),
    )
        .await?;

    consume_queue(
        &channel,
        &config.protobuf_queue,
        MessageFormat::Protobuf,
        repository.clone(),
    )
        .await?;

    println!("Benchmark worker is running...");
    println!("Listening queues:");
    println!(" - {}", config.json_queue);
    println!(" - {}", config.protobuf_queue);

    futures_util::future::pending::<()>().await;
    Ok(())
}

async fn setup_benchmark_topology(
    channel: &Channel,
    config: &BenchmarkWorkerConfig,
) -> Result<(), AppError> {
    channel
        .exchange_declare(
            &config.exchange,
            ExchangeKind::Direct,
            ExchangeDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(|e| AppError::RabbitMq(e.to_string()))?;

    channel
        .queue_declare(
            &config.json_queue,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(|e| AppError::RabbitMq(e.to_string()))?;

    channel
        .queue_declare(
            &config.protobuf_queue,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(|e| AppError::RabbitMq(e.to_string()))?;

    channel
        .queue_bind(
            &config.json_queue,
            &config.exchange,
            MessageFormat::Json.routing_key(),
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(|e| AppError::RabbitMq(e.to_string()))?;

    channel
        .queue_bind(
            &config.protobuf_queue,
            &config.exchange,
            MessageFormat::Protobuf.routing_key(),
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(|e| AppError::RabbitMq(e.to_string()))?;

    Ok(())
}

async fn consume_queue(
    channel: &Channel,
    queue_name: &str,
    format: MessageFormat,
    repository: Arc<BenchmarkRepository>,
) -> Result<(), AppError> {
    let mut consumer = channel
        .basic_consume(
            queue_name,
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(|e| AppError::RabbitMq(e.to_string()))?;

    tokio::spawn(async move {
        while let Some(delivery_result) = consumer.next().await {
            match delivery_result {
                Ok(delivery) => {
                    let payload_size = delivery.data.len();

                    match deserialize_event(&delivery.data, format) {
                        Ok(event) => {
                            let metrics = collect_metrics(&event, format, payload_size);

                            if let Err(err) = repository.save_result(&metrics).await {
                                eprintln!("Failed to save benchmark result: {}", err);
                            } else {
                                println!(
                                    "Saved benchmark result: event_id={}, format={}, latency_ms={}, payload_size={}",
                                    metrics.event_id,
                                    metrics.format,
                                    metrics.latency_ms,
                                    metrics.payload_size_bytes
                                );
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to deserialize message: {}", err);
                        }
                    }

                    if let Err(err) = delivery.ack(BasicAckOptions::default()).await {
                        eprintln!("Failed to ack message: {}", err);
                    }
                }
                Err(err) => {
                    eprintln!("Consumer delivery error: {}", err);
                }
            }
        }
    });

    Ok(())
}