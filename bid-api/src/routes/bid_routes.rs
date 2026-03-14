
use axum::{
    routing::{get, post},
    Router,
};

use crate::config::AppState;

use crate::auth::google::{google_callback, google_login};

use crate::handlers::{
    place_bid_handler::{health_handler, place_bid_handler},
    user_handler::me_handler,};

pub fn create_bid_routes(state: AppState) -> Router {
    Router::new()
        .route("/auth/google/login", get(google_login))
        .route("/auth/google/callback", get(google_callback))
        .route("/health", get(health_handler))
        .route("/api/bids", post(place_bid_handler))
        .route("/api/users/me", get(me_handler))
        .with_state(state)
}