
use chrono::{DateTime, Utc};
use std::str::FromStr;
use uuid::Uuid;

use shared::{
    error::AppError,
    events::BidPlacedEvent,
    types::Currency,
};

#[derive(Debug, Clone)]
pub struct BidEntity {
    pub id: Uuid,
    pub lot_id: Uuid,
    pub bidder_id: Uuid,
    pub amount: f64,
    pub currency: Currency,
    pub created_at: DateTime<Utc>,
}

impl BidEntity {
    pub fn from_event(event: BidPlacedEvent) -> Result<Self, AppError> {
        let id = Uuid::parse_str(&event.bid_id)
            .map_err(|e| AppError::Validation(format!("Invalid bid_id UUID: {}", e)))?;

        let lot_id = Uuid::parse_str(&event.lot_id)
            .map_err(|e| AppError::Validation(format!("Invalid lot_id UUID: {}", e)))?;

        let bidder_id = Uuid::parse_str(&event.user_id)
            .map_err(|e| AppError::Validation(format!("Invalid user_id UUID: {}", e)))?;

        let currency = Currency::from_str(&event.currency)
            .map_err(AppError::Validation)?;

        Ok(Self {
            id,
            lot_id,
            bidder_id,
            amount: event.amount,
            currency,
            created_at: event.occurred_at,
        })
    }
}