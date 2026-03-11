<<<<<<< HEAD
use async_trait::async_trait;

use shared::error::AppError;

use crate::domain::bid_entity::BidEntity;

#[async_trait]
pub trait BidRepository {
    async fn save_bid(&self, bid: &BidEntity) -> Result<(), AppError>;
=======
use async_trait::async_trait;

use shared::error::AppError;

use crate::domain::bid_entity::BidEntity;

#[async_trait]
pub trait BidRepository {
    async fn save_bid(&self, bid: &BidEntity) -> Result<(), AppError>;
>>>>>>> f63628c3a44df1d65bd9e805f0eef628cd04195e
}