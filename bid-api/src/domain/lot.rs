use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Lot {
    pub id: Uuid,
    pub seller_id: Uuid,
    pub title: String,
    pub description: String,
    pub start_price: f64,
    pub min_increment: f64,
    pub currency: String,
    pub start_at_utc: NaiveDateTime,
    pub end_at_utc: NaiveDateTime,
    pub status: String,
    pub created_at_utc: NaiveDateTime,
}