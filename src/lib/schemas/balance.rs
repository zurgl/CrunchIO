use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Balance {
    amount: f64,
    currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Currency {
    USD,
}
