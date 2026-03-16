mod application;
mod auth;
mod config;
mod domain;
mod handlers;
mod infrastructure;
mod presentation;
mod routes;

use std::{path::Path, sync::Arc};

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
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

use crate::config::AppState;
use crate::infrastructure::event_publisher::RabbitMqEventPublisher;
use crate::routes::bid_routes::create_bid_routes;
use crate::routes::lot_routes::lot_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = AppConfig::from_env();

    let publisher = RabbitMqEventPublisher::new(&config)
        .await
        .expect("Failed to initialize RabbitMQ publisher");

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    if !Path::new("uploads/lots").exists() {
        std::fs::create_dir_all("uploads/lots").expect("Failed to create uploads/lots");
    }

    let state = AppState {
        config: config.clone(),
        publisher: Arc::new(publisher),
        db,
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5174".parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let app = Router::new()
        .nest_service(
            "/uploads",
            ServeDir::new("uploads").not_found_service(ServeFile::new("uploads/no-image.png")),
        )
        .merge(create_bid_routes(state.clone()))
        .merge(lot_routes(state.clone()))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(config.api_addr())
        .await
        .expect("Failed to bind TCP listener");

    println!("bid-api is running on {}", config.api_addr());

    axum::serve(listener, app)
        .await
        .expect("Failed to start axum server");
}