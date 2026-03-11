
use async_trait::async_trait;

use shared::error::AppError;

use crate::domain::bid_entity::BidEntity;

#[async_trait]
pub trait BidRepository {
    async fn save_bid(&self, bid: &BidEntity) -> Result<(), AppError>;
}