use crate::domain::lot::Lot;
use crate::infrastructure::lot_repository::LotRepository;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct CreateLotUseCase;

impl CreateLotUseCase {
    pub async fn execute(
        pool: &PgPool,
        seller_id: Uuid,
        title: String,
        description: String,
        start_price: f64,
        auction_end: DateTime<Utc>,
        image_url: Option<String>,
    ) -> Result<Lot, sqlx::Error> {
        LotRepository::create(
            pool,
            seller_id,
            title,
            description,
            start_price,
            auction_end,
            image_url,
        )
            .await
    }
}