
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub type TimestampMs = i64;
pub type PayloadSizeBytes = usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Currency {
    UAH,
    USD,
    EUR,
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::UAH => write!(f, "UAH"),
            Currency::USD => write!(f, "USD"),
            Currency::EUR => write!(f, "EUR"),
        }
    }
}

impl FromStr for Currency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_uppercase().as_str() {
            "UAH" => Ok(Currency::UAH),
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            other => Err(format!("Unsupported currency: {}", other)),
        }
    }
}