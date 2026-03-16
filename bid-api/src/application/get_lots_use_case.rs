use crate::domain::lot::Lot;
use crate::infrastructure::lot_repository::LotRepository;
use sqlx::PgPool;

pub struct GetLotsUseCase;

impl GetLotsUseCase {
    pub async fn execute(pool: &PgPool) -> Result<Vec<Lot>, sqlx::Error> {
        LotRepository::get_all(pool).await
    }
}