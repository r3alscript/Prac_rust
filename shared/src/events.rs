
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidPlacedEvent {
    pub event_id: String,
    pub event_type: String,
    pub occurred_at: DateTime<Utc>,
    pub bid_id: String,
    pub lot_id: String,
    pub user_id: String,
    pub amount: f64,
    pub currency: String,
    pub sent_time_ms: i64,
}

impl BidPlacedEvent {
    pub fn new(
        bid_id: impl Into<String>,
        lot_id: impl Into<String>,
        user_id: impl Into<String>,
        amount: f64,
        currency: impl Into<String>,
    ) -> Self {
        let now = Utc::now();

        Self {
            event_id: Uuid::new_v4().to_string(),
            event_type: "BidPlacedEvent".to_string(),
            occurred_at: now,
            bid_id: bid_id.into(),
            lot_id: lot_id.into(),
            user_id: user_id.into(),
            amount,
            currency: currency.into(),
            sent_time_ms: now.timestamp_millis(),
        }
    }
}