pub type PlaceBidRequest = shared::dto::PlaceBidRequest;
pub type PlaceBidResponse = shared::dto::PlaceBidResponse;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLotRequest {
    pub title: String,
    pub description: String,
    pub start_price: f64,
    pub auction_end: DateTime<Utc>,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBidRequest {
    pub amount: f64,
}