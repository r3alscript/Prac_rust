<<<<<<< HEAD
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::Currency;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceBidRequest {
    pub lot_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub currency: Currency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceBidResponse {
    pub status: String,
    pub bid_id: Uuid,
    pub message: String,
=======
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::Currency;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceBidRequest {
    pub lot_id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub currency: Currency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceBidResponse {
    pub status: String,
    pub bid_id: Uuid,
    pub message: String,
>>>>>>> f63628c3a44df1d65bd9e805f0eef628cd04195e
}