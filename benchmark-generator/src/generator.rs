
use shared::events::BidPlacedEvent;
use uuid::Uuid;

pub fn generate_bid_event(index: usize) -> BidPlacedEvent {
    BidPlacedEvent {
        event_id: Uuid::new_v4().to_string(),
        event_type: "BidPlacedEvent".to_string(),
        occurred_at: chrono::Utc::now(),
        bid_id: Uuid::new_v4().to_string(),
        lot_id: Uuid::new_v4().to_string(),
        user_id: Uuid::new_v4().to_string(),
        amount: 1000.0 + index as f64,
        currency: "UAH".to_string(),
        sent_time_ms: chrono::Utc::now().timestamp_millis(),
    }
}