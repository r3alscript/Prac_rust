<<<<<<< HEAD
use std::sync::Arc;

use shared::config::AppConfig;

use crate::infrastructure::event_publisher::RabbitMqEventPublisher;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub publisher: Arc<RabbitMqEventPublisher>,
=======
use std::sync::Arc;

use shared::config::AppConfig;

use crate::infrastructure::event_publisher::RabbitMqEventPublisher;

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub publisher: Arc<RabbitMqEventPublisher>,
>>>>>>> f63628c3a44df1d65bd9e805f0eef628cd04195e
}