
use crate::error::AppError;
use crate::events::BidPlacedEvent;
use crate::proto::auction::BidPlacedEventProto;
use chrono::{DateTime, Utc};
use prost::Message;

pub fn serialize(event: &BidPlacedEvent) -> Result<Vec<u8>, AppError> {
    let proto = BidPlacedEventProto {
        event_id: event.event_id.clone(),
        event_type: event.event_type.clone(),
        occurred_at: event.occurred_at.to_rfc3339(),
        bid_id: event.bid_id.clone(),
        lot_id: event.lot_id.clone(),
        user_id: event.user_id.clone(),
        amount: event.amount,
        currency: event.currency.clone(),
        sent_time_ms: event.sent_time_ms,
    };

    let mut buf = Vec::new();
    proto.encode(&mut buf).map_err(AppError::ProtobufEncode)?;
    Ok(buf)
}

pub fn deserialize(payload: &[u8]) -> Result<BidPlacedEvent, AppError> {
    let proto =
        BidPlacedEventProto::decode(payload).map_err(AppError::ProtobufDecode)?;

    let occurred_at = DateTime::parse_from_rfc3339(&proto.occurred_at)
        .map_err(|e| AppError::Serialization(format!("Invalid occurred_at: {}", e)))?
        .with_timezone(&Utc);

    Ok(BidPlacedEvent {
        event_id: proto.event_id,
        event_type: proto.event_type,
        occurred_at,
        bid_id: proto.bid_id,
        lot_id: proto.lot_id,
        user_id: proto.user_id,
        amount: proto.amount,
        currency: proto.currency,
        sent_time_ms: proto.sent_time_ms,
    })
}