#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum VolumeStatus {
  #[default]
  Attached,
  Detached,
  Deleted,
  Ordered,
  Creating,
  Deleting,
}
