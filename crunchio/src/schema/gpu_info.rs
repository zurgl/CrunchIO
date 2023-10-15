#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct GpuInfo {
  description: String,
  number_of_gpus: u64,
}
