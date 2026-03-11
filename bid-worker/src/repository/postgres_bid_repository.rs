<<<<<<< HEAD
use async_trait::async_trait;
use sqlx::PgPool;

use shared::error::AppError;

use crate::{
    domain::bid_entity::BidEntity,
    infrastructure::bid_repository::BidRepository,
};

pub struct PostgresBidRepository {
    pool: PgPool,
}

impl PostgresBidRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BidRepository for PostgresBidRepository {
    async fn save_bid(&self, bid: &BidEntity) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO bids (id, lot_id, bidder_id, amount, currency, placed_at_utc)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
            .bind(bid.id)
            .bind(bid.lot_id)
            .bind(bid.bidder_id)
            .bind(bid.amount)
            .bind(bid.currency.to_string())
            .bind(bid.created_at.naive_utc())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
=======
use async_trait::async_trait;
use sqlx::PgPool;

use shared::error::AppError;

use crate::{
    domain::bid_entity::BidEntity,
    infrastructure::bid_repository::BidRepository,
};

pub struct PostgresBidRepository {
    pool: PgPool,
}

impl PostgresBidRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BidRepository for PostgresBidRepository {
    async fn save_bid(&self, bid: &BidEntity) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO bids (id, lot_id, bidder_id, amount, currency, placed_at_utc)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
            .bind(bid.id)
            .bind(bid.lot_id)
            .bind(bid.bidder_id)
            .bind(bid.amount)
            .bind(bid.currency.to_string())
            .bind(bid.created_at.naive_utc())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
>>>>>>> f63628c3a44df1d65bd9e805f0eef628cd04195e
}