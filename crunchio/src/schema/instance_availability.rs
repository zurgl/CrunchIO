#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Availability {
  pub location_code: super::LocationCode,
  pub availabilities: Vec<String>,
}
