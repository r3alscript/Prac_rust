mod application;
mod auth;
mod config;
mod domain;
mod handlers;
mod infrastructure;
mod presentation;
mod routes;

use std::sync::Arc;

use axum::{
    Router,
    http::{
        HeaderValue,
        Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};
use dotenvy::dotenv;
use shared::config::AppConfig;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

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

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    let state = AppState {
        config: config.clone(),
        publisher: Arc::new(publisher),
        db,
    };

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5174".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let app: Router = create_bid_routes(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(config.api_addr())
        .await
        .expect("Failed to bind TCP listener");

    println!("bid-api is running on {}", config.api_addr());

    axum::serve(listener, app)
        .await
        .expect("Failed to start axum server");
}