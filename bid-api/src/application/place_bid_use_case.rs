
use shared::{
    dto::{PlaceBidRequest, PlaceBidResponse},
    error::AppError,
};

use crate::{
    config::AppState,
    domain::bid::Bid,
};

pub async fn place_bid(
    state: &AppState,
    request: PlaceBidRequest,
) -> Result<PlaceBidResponse, AppError> {
    if request.amount <= 0.0 {
        return Err(AppError::Validation(
            "Bid amount must be greater than 0".to_string(),
        ));
    }

    let bid = Bid::new(
        request.lot_id,
        request.user_id,
        request.amount,
        request.currency,
    );

    let event = bid.to_bid_placed_event();

    state
        .publisher
        .publish_bid_placed(&event)
        .await?;

    Ok(PlaceBidResponse {
        status: "accepted".to_string(),
        bid_id: bid.id,
        message: "Bid accepted for asynchronous processing".to_string(),
    })
}