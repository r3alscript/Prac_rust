use axum::{
    routing::{get, post},
    Router,
};

use crate::config::AppState;
use crate::handlers::place_bid_handler::{health_handler, place_bid_handler};

pub fn create_bid_routes(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/api/bids", post(place_bid_handler))
        .with_state(state)
}