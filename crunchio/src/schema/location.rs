#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Location {
  pub code: super::LocationCode,
  pub name: String,
  pub country_code: String,
}
