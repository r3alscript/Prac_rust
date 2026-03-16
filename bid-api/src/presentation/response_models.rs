use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct LotCardResponse {
    pub id: Uuid,
    pub title: String,
    pub start_price: f64,
    pub current_price: f64,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LotDetailsResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub start_price: f64,
    pub current_price: f64,
    pub seller_id: Uuid,
    pub image_url: Option<String>,
    pub auction_end: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub bids_count: i64,
    pub max_bid: f64,
}