use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;

use crate::{
    handlers::dashboard_handler::{
        get_dashboard_handler,
        get_progress_handler,
        run_full_benchmark_handler,
        run_json_benchmark_handler,
        run_protobuf_benchmark_handler,
    },
    AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/benchmark/dashboard", get(get_dashboard_handler))
        .route("/api/benchmark/progress", get(get_progress_handler))
        .route("/api/benchmark/run/json", post(run_json_benchmark_handler))
        .route("/api/benchmark/run/protobuf", post(run_protobuf_benchmark_handler))
        .route("/api/benchmark/run/full", post(run_full_benchmark_handler))
        .layer(CorsLayer::permissive())
        .with_state(state)
}