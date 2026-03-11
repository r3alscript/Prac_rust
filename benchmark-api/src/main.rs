mod dto;
mod handlers;
mod repository;
mod router;
mod service;

use sqlx::PgPool;
use std::net::SocketAddr;

use crate::{
    repository::benchmark_repository::BenchmarkRepository,
    router::create_router,
    service::benchmark_service::BenchmarkService,
};

#[derive(Clone)]
pub struct AppState {
    pub benchmark_service: BenchmarkService,
}

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@postgres:5432/auction_db".to_string());

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    let repository = BenchmarkRepository::new(pool);
    let benchmark_service = BenchmarkService::new(repository);

    let state = AppState { benchmark_service };

    let app = create_router(state);

    let addr: SocketAddr = std::env::var("BENCHMARK_API_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:8081".to_string())
        .parse()
        .expect("Invalid bind address");

    println!("benchmark-api started on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to start benchmark-api");
}