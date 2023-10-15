use crate::utils::deserialize_null_default;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct InstanceType {
  pub best_for: Vec<String>,
  pub cpu: super::CpuInfo,
  #[serde(deserialize_with = "deserialize_null_default")]
  pub deploy_warning: String,
  pub description: String,
  pub gpu: super::GpuInfo,
  pub gpu_memory: super::Memory,
  pub id: uuid::Uuid,
  pub instance_type: String,
  pub memory: super::Memory,
  pub model: String,
  pub name: String,
  pub p2p: String,
  pub price_per_hour: String,
  pub spot_price: String,
  pub storage: super::StorageType,
}
