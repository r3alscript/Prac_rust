
use futures_util::StreamExt;
use lapin::{
    options::{
        BasicAckOptions,
        BasicConsumeOptions,
        ExchangeDeclareOptions,
        QueueBindOptions,
        QueueDeclareOptions,
    },
    types::FieldTable,
    Connection, ConnectionProperties, ExchangeKind,
};
use shared::{
    config::AppConfig,
    error::AppError,
    events::BidPlacedEvent,
    rabbitmq::BID_EXCHANGE_KIND,
};

use crate::{
    application::process_bid_event::process_bid_event,
    infrastructure::bid_repository::BidRepository,
};

pub async fn run_bid_consumer<R>(
    config: &AppConfig,
    repository: R,
) -> Result<(), AppError>
where
    R: BidRepository + Send + Sync,
{
    let conn = Connection::connect(&config.rabbitmq_url, ConnectionProperties::default())
        .await
        .map_err(|e| AppError::RabbitMq(e.to_string()))?;

    let channel = conn
        .create_channel()
        .await
        .map_err(|e| AppError::RabbitMq(e.to_string()))?;

    let exchange_kind = match BID_EXCHANGE_KIND {
        "direct" => ExchangeKind::Direct,
        "fanout" => ExchangeKind::Fanout,
        "topic" => ExchangeKind::Topic,
        _ => ExchangeKind::Direct,
    };

    channel
        .exchange_declare(
            &config.bid_exchange,
            exchange_kind,
            ExchangeDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(|e| AppError::RabbitMq(e.to_string()))?;

    channel
        .queue_declare(
            &config.bid_queue,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(|e| AppError::RabbitMq(e.to_string()))?;

    channel
        .queue_bind(
            &config.bid_queue,
            &config.bid_exchange,
            &config.bid_routing_key,
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(|e| AppError::RabbitMq(e.to_string()))?;

    let mut consumer = channel
        .basic_consume(
            &config.bid_queue,
            "bid-worker-consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .map_err(|e| AppError::RabbitMq(e.to_string()))?;

    println!(
        "bid-worker is listening queue '{}' on exchange '{}'",
        config.bid_queue, config.bid_exchange
    );

    while let Some(delivery_result) = consumer.next().await {
        let delivery = delivery_result.map_err(|e| AppError::RabbitMq(e.to_string()))?;

        let event: BidPlacedEvent = serde_json::from_slice(&delivery.data)
            .map_err(|e| AppError::Serialization(e.to_string()))?;

        println!(
            "Received event {} for lot {} and user {}",
            event.event_id, event.lot_id, event.user_id
        );

        process_bid_event(&repository, event).await?;

        delivery
            .ack(BasicAckOptions::default())
            .await
            .map_err(|e| AppError::RabbitMq(e.to_string()))?;

        println!("Message processed and acknowledged");
    }

    Ok(())
}