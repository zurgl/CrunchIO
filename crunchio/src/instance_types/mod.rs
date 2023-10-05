use crate::_shared_::deserialize_null_default;

use super::{CpuInfo, CrunchIO, Error, GpuInfo, Memory, QueryParams, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StorageType {
  description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstanceType {
  pub best_for: Vec<String>,
  pub cpu: CpuInfo,
  #[serde(deserialize_with = "deserialize_null_default")]
  pub deploy_warning: String,
  pub description: String,
  pub gpu: GpuInfo,
  pub gpu_memory: Memory,
  pub id: Uuid,
  pub instance_type: String,
  pub memory: Memory,
  pub model: String,
  pub name: String,
  pub p2p: String,
  pub price_per_hour: String,
  pub spot_price: String,
  pub storage: StorageType,
}

use super::routes::INSTANCE_TYPES as path;

pub type InstanceTypes = Vec<InstanceType>;

impl CrunchIO {
  pub fn get_all_instance_types(&self) -> Result<InstanceTypes> {
    self
      .public_http_request(&QueryParams {
        path,
        ..Default::default()
      })?
      .into_json()
      .map_err(Error::JsonParsing)
  }
}
