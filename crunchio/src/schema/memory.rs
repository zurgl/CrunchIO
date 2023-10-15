#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct Memory {
  description: String,
  size_in_gigabytes: u64,
}
