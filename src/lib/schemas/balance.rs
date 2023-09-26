#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Balance {
    amount: f64,
    currency: String,
}
