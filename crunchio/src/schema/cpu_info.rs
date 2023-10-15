#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct CpuInfo {
  description: String,
  number_of_cores: u64,
}
