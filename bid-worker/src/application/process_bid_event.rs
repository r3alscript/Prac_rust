
use shared::{error::AppError, events::BidPlacedEvent};

use crate::{domain::bid_entity::BidEntity, infrastructure::bid_repository::BidRepository};

pub async fn process_bid_event<R>(
    repository: &R,
    event: BidPlacedEvent,
) -> Result<(), AppError>
where
    R: BidRepository + Send + Sync,
{
    let bid = BidEntity::from_event(event)?;
    repository.save_bid(&bid).await?;
    Ok(())
}