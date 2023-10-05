use super::{CrunchIO, Error, QueryParams, Result};
use crate::{balance::Currency, volumes::VolumeType};
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

use super::routes::VOLUME_TYPES as path;

impl CrunchIO {
  pub fn get_volumes_types(&self) -> Result<VolumeTypeInfos> {
    self
      .http_request(&QueryParams {
        path,
        ..Default::default()
      })?
      .into_json()
      .map_err(Error::JsonParsing)
  }
}
