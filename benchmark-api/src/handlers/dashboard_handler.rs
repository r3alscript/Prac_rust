use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use crate::{
    dto::dashboard_response::{DashboardResponse, RunBenchmarkResponse},
    service::benchmark_service::BenchmarkProgressState,
    AppState,
};

pub async fn get_dashboard_handler(
    State(state): State<AppState>,
) -> Result<Json<DashboardResponse>, (StatusCode, String)> {
    state
        .benchmark_service
        .get_dashboard()
        .await
        .map(Json)
        .map_err(internal_error)
}

pub async fn get_progress_handler(
    State(state): State<AppState>,
) -> Result<Json<BenchmarkProgressState>, (StatusCode, String)> {
    Ok(Json(state.benchmark_service.get_progress().await))
}

pub async fn run_json_benchmark_handler(
    State(state): State<AppState>,
) -> Result<Json<RunBenchmarkResponse>, (StatusCode, String)> {
    state
        .benchmark_service
        .start_json_benchmark()
        .await
        .map(Json)
        .map_err(internal_error)
}

pub async fn run_protobuf_benchmark_handler(
    State(state): State<AppState>,
) -> Result<Json<RunBenchmarkResponse>, (StatusCode, String)> {
    state
        .benchmark_service
        .start_protobuf_benchmark()
        .await
        .map(Json)
        .map_err(internal_error)
}

pub async fn run_full_benchmark_handler(
    State(state): State<AppState>,
) -> Result<Json<RunBenchmarkResponse>, (StatusCode, String)> {
    state
        .benchmark_service
        .start_full_benchmark()
        .await
        .map(Json)
        .map_err(internal_error)
}

fn internal_error(error: impl ToString) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
}