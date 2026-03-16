use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    config::AppState,
    handlers::lot_handler::{
        create_lot_handler,
        get_lot_by_id_handler,
        get_lots_handler,
        place_bid_handler,
    },
};

pub fn lot_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/lots", get(get_lots_handler).post(create_lot_handler))
        .route("/api/lots/:id", get(get_lot_by_id_handler))
        .route("/api/lots/:id/bids", post(place_bid_handler))
        .with_state(state)
}