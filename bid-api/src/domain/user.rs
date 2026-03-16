use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub surname: String,
    pub photo_url: Option<String>,
    pub balance: Decimal,
    pub created_at_utc: chrono::NaiveDateTime,
}