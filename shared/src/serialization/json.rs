use crate::error::AppError;
use crate::events::BidPlacedEvent;

pub fn serialize(event: &BidPlacedEvent) -> Result<Vec<u8>, AppError> {
    serde_json::to_vec(event).map_err(AppError::SerdeJson)
}

pub fn deserialize(payload: &[u8]) -> Result<BidPlacedEvent, AppError> {
    serde_json::from_slice(payload).map_err(AppError::SerdeJson)
}