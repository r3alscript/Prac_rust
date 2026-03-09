use lapin::{
    options::{BasicPublishOptions, ConfirmSelectOptions, ExchangeDeclareOptions},
    publisher_confirm::Confirmation,
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind,
};
use shared::{
    error::AppError,
    events::BidPlacedEvent,
    serialization::{format::MessageFormat, serialize_event},
};

pub struct BenchmarkPublisher {
    channel: Channel,
    exchange: String,
}

impl BenchmarkPublisher {
    pub async fn new(rabbitmq_url: &str, exchange: &str) -> Result<Self, AppError> {
        let connection = Connection::connect(rabbitmq_url, ConnectionProperties::default())
            .await
            .map_err(|e| AppError::RabbitMq(e.to_string()))?;

        let channel = connection
            .create_channel()
            .await
            .map_err(|e| AppError::RabbitMq(e.to_string()))?;

        channel
            .confirm_select(ConfirmSelectOptions::default())
            .await
            .map_err(|e| AppError::RabbitMq(e.to_string()))?;

        channel
            .exchange_declare(
                exchange,
                ExchangeKind::Direct,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| AppError::RabbitMq(e.to_string()))?;

        Ok(Self {
            channel,
            exchange: exchange.to_string(),
        })
    }

    pub async fn publish(
        &self,
        event: &BidPlacedEvent,
        format: MessageFormat,
    ) -> Result<usize, AppError> {
        let payload = serialize_event(event, format)?;
        let payload_size = payload.len();

        let confirm: Confirmation = self
            .channel
            .basic_publish(
                &self.exchange,
                format.routing_key(),
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default().with_content_type(format.content_type().into()),
            )
            .await
            .map_err(|e| AppError::RabbitMq(e.to_string()))?
            .await
            .map_err(|e| AppError::RabbitMq(e.to_string()))?;

        if !confirm.is_ack() {
            return Err(AppError::RabbitMq(
                "Message publish was not acknowledged".to_string(),
            ));
        }

        Ok(payload_size)
    }
}