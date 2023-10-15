#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub enum VolumeType {
  #[default]
  NVMe,
  HDD,
}
