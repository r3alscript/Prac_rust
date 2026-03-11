<<<<<<< HEAD
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use shared::dto::{PlaceBidRequest, PlaceBidResponse};

use crate::application::place_bid_use_case::place_bid;
use crate::config::AppState;

pub async fn health_handler() -> &'static str {
    "OK"
}

pub async fn place_bid_handler(
    State(state): State<AppState>,
    Json(request): Json<PlaceBidRequest>,
) -> Result<(StatusCode, Json<PlaceBidResponse>), (StatusCode, String)> {
    let response = place_bid(&state, request)
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;

    Ok((StatusCode::ACCEPTED, Json(response)))
=======
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use shared::dto::{PlaceBidRequest, PlaceBidResponse};

use crate::application::place_bid_use_case::place_bid;
use crate::config::AppState;

pub async fn health_handler() -> &'static str {
    "OK"
}

pub async fn place_bid_handler(
    State(state): State<AppState>,
    Json(request): Json<PlaceBidRequest>,
) -> Result<(StatusCode, Json<PlaceBidResponse>), (StatusCode, String)> {
    let response = place_bid(&state, request)
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;

    Ok((StatusCode::ACCEPTED, Json(response)))
>>>>>>> f63628c3a44df1d65bd9e805f0eef628cd04195e
}