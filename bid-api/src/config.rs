use std::sync::Arc;

use shared::config::AppConfig;
use sqlx::PgPool;

use crate::infrastructure::event_publisher::RabbitMqEventPublisher;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub publisher: Arc<RabbitMqEventPublisher>,
    pub db: PgPool,
}