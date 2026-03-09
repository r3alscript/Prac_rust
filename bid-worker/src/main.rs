mod application;
mod config;
mod consumer;
mod domain;
mod infrastructure;
mod repository;

use dotenvy::dotenv;
use shared::config::AppConfig;

use crate::consumer::bid_consumer::run_bid_consumer;
use crate::infrastructure::db::create_db_pool;
use crate::repository::postgres_bid_repository::PostgresBidRepository;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = AppConfig::from_env();

    let pool = create_db_pool(&config.database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    let repository = PostgresBidRepository::new(pool);

    println!("bid-worker is starting...");

    run_bid_consumer(&config, repository)
        .await
        .expect("bid-worker failed");
}