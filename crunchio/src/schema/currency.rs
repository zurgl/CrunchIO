#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Currency {
  #[serde(rename = "usd")]
  USD,
}
