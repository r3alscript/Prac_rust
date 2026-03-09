use lapin::{
    options::{
        BasicPublishOptions,
        ExchangeDeclareOptions,
        QueueBindOptions,
        QueueDeclareOptions,
    },
    publisher_confirm::Confirmation,
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind,
};

use shared::{
    config::AppConfig,
    error::AppError,
    events::BidPlacedEvent,
    rabbitmq::{BID_CONTENT_TYPE_JSON, BID_EXCHANGE_KIND},
};

pub struct RabbitMqEventPublisher {
    channel: Channel,
    exchange: String,
    queue: String,
    routing_key: String,
}

impl RabbitMqEventPublisher {
    pub async fn new(config: &AppConfig) -> Result<Self, AppError> {
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

        Ok(Self {
            channel,
            exchange: config.bid_exchange.clone(),
            queue: config.bid_queue.clone(),
            routing_key: config.bid_routing_key.clone(),
        })
    }

    pub async fn publish_bid_placed(&self, event: &BidPlacedEvent) -> Result<(), AppError> {
        let payload =
            serde_json::to_vec(event).map_err(|e| AppError::Serialization(e.to_string()))?;

        let confirm: Confirmation = self
            .channel
            .basic_publish(
                &self.exchange,
                &self.routing_key,
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default()
                    .with_content_type(BID_CONTENT_TYPE_JSON.into())
                    .with_type(event.event_type.clone().into()),
            )
            .await
            .map_err(|e| AppError::RabbitMq(e.to_string()))?
            .await
            .map_err(|e| AppError::RabbitMq(e.to_string()))?;

        if confirm.is_nack() {
            return Err(AppError::RabbitMq(
                "RabbitMQ returned NACK for published message".to_string(),
            ));
        }

        println!(
            "Published event {} to exchange '{}' with routing key '{}' (queue '{}')",
            event.event_id, self.exchange, self.routing_key, self.queue
        );

        Ok(())
    }
}