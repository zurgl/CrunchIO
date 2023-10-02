use crate::{balance::Currency, volumes::VolumeType};

use super::{CrunchIO, QueryParams};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceInfo {
  pub currency: Currency,
  pub price_per_month_per_gb: f32,
  // "cps_per_gb": 1.90258751902588e-8,
  // pub cps_per_gb: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VolumeTypeInfo {
  #[serde(rename(deserialize = "type"))]
  pub volume_type: VolumeType,
  pub price: PriceInfo,
}

pub type VolumeTypeInfos = Vec<VolumeTypeInfo>;

const PATH: &str = "volume-types";

impl CrunchIO {
  pub fn get_volumes_types(&self) -> VolumeTypeInfos {
    match self
      .query(&QueryParams {
        path: PATH,
        ..Default::default()
      })
      .into_json()
    {
      Ok(volumes) => volumes,
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }
}
