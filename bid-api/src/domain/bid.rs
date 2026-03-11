<<<<<<< HEAD
use chrono::{DateTime, Utc};
use uuid::Uuid;

use shared::{
    events::BidPlacedEvent,
    types::Currency,
};

#[derive(Debug, Clone)]
pub struct Bid {
    pub id: Uuid,
    pub lot_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub currency: Currency,
    pub created_at: DateTime<Utc>,
}

impl Bid {
    pub fn new(
        lot_id: Uuid,
        user_id: Uuid,
        amount: f64,
        currency: Currency,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            lot_id,
            user_id,
            amount,
            currency,
            created_at: Utc::now(),
        }
    }

    pub fn to_bid_placed_event(&self) -> BidPlacedEvent {
        BidPlacedEvent {
            event_id: Uuid::new_v4().to_string(),
            event_type: "BidPlacedEvent".to_string(),
            occurred_at: self.created_at,
            bid_id: self.id.to_string(),
            lot_id: self.lot_id.to_string(),
            user_id: self.user_id.to_string(),
            amount: self.amount,
            currency: self.currency.to_string(),
            sent_time_ms: Utc::now().timestamp_millis(),
        }
    }
=======
use chrono::{DateTime, Utc};
use uuid::Uuid;

use shared::{
    events::BidPlacedEvent,
    types::Currency,
};

#[derive(Debug, Clone)]
pub struct Bid {
    pub id: Uuid,
    pub lot_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub currency: Currency,
    pub created_at: DateTime<Utc>,
}

impl Bid {
    pub fn new(
        lot_id: Uuid,
        user_id: Uuid,
        amount: f64,
        currency: Currency,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            lot_id,
            user_id,
            amount,
            currency,
            created_at: Utc::now(),
        }
    }

    pub fn to_bid_placed_event(&self) -> BidPlacedEvent {
        BidPlacedEvent {
            event_id: Uuid::new_v4().to_string(),
            event_type: "BidPlacedEvent".to_string(),
            occurred_at: self.created_at,
            bid_id: self.id.to_string(),
            lot_id: self.lot_id.to_string(),
            user_id: self.user_id.to_string(),
            amount: self.amount,
            currency: self.currency.to_string(),
            sent_time_ms: Utc::now().timestamp_millis(),
        }
    }
>>>>>>> f63628c3a44df1d65bd9e805f0eef628cd04195e
}