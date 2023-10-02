use crate::_shared_::deserialize_null_default;

use super::{CpuInfo, CrunchIO, GpuInfo, Memory, QueryParams};
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

const PATH: &str = "instance-types";

pub type InstanceTypes = Vec<InstanceType>;

impl CrunchIO {
  pub fn get_all_instance_types(&self) -> InstanceTypes {
    match self
      .public_query(&QueryParams {
        path: PATH,
        ..Default::default()
      })
      .into_json()
    {
      Ok(instances) => instances,
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }
}
