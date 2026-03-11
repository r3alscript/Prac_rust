pub mod format;
pub mod json;
pub mod protobuf;

use crate::error::AppError;
use crate::events::BidPlacedEvent;
use format::MessageFormat;

pub fn serialize_event(
    event: &BidPlacedEvent,
    format: MessageFormat,
) -> Result<Vec<u8>, AppError> {
    match format {
        MessageFormat::Json => json::serialize(event),
        MessageFormat::Protobuf => protobuf::serialize(event),
    }
}

pub fn deserialize_event(
    payload: &[u8],
    format: MessageFormat,
) -> Result<BidPlacedEvent, AppError> {
    match format {
        MessageFormat::Json => json::deserialize(payload),
        MessageFormat::Protobuf => protobuf::deserialize(payload),
    }
}