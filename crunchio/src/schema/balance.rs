#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct UserBalance {
  pub amount: f64,
  pub currency: super::Currency,
}
