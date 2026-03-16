use crate::domain::lot::Lot;
use crate::infrastructure::lot_repository::LotRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub struct GetLotUseCase;

impl GetLotUseCase {
    pub async fn execute(pool: &PgPool, lot_id: Uuid) -> Result<Option<Lot>, sqlx::Error> {
        LotRepository::get_by_id(pool, lot_id).await
    }
}