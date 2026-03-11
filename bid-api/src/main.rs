mod application;
mod config;
mod domain;
mod handlers;
mod infrastructure;
mod presentation;
mod routes;

use std::sync::Arc;

use axum::Router;
use dotenvy::dotenv;
use shared::config::AppConfig;

use crate::config::AppState;
use crate::infrastructure::event_publisher::RabbitMqEventPublisher;
use crate::routes::bid_routes::create_bid_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = AppConfig::from_env();

    let publisher = RabbitMqEventPublisher::new(&config)
        .await
        .expect("Failed to initialize RabbitMQ publisher");

    let state = AppState {
        config: config.clone(),
        publisher: Arc::new(publisher),
    };

    let app: Router = create_bid_routes(state);

    let listener = tokio::net::TcpListener::bind(config.api_addr())
        .await
        .expect("Failed to bind TCP listener");

    println!("bid-api is running on {}", config.api_addr());

    axum::serve(listener, app)
        .await
        .expect("Failed to start axum server");
}