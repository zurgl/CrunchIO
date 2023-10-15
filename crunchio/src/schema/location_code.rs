#[derive(
  serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, Eq,
)]
pub enum LocationCode {
  #[default]
  #[serde(rename = "FIN-01")]
  FIN01,
  #[serde(rename = "ICE-01")]
  ICE01,
}

impl From<&LocationCode> for &str {
  fn from(value: &LocationCode) -> Self {
    match value {
      LocationCode::FIN01 => "FIN-01",
      LocationCode::ICE01 => "ICE-01",
    }
  }
}
