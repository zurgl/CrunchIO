use super::{locations::LocationCode, CrunchIO, QueryParams};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstancesAvailability {
  pub location_code: LocationCode,
  pub availabilities: Vec<String>,
}

pub type InstancesAvailabilities = Vec<InstancesAvailability>;

use super::routes::INSTANCE_AVAILABILITY as path;

impl CrunchIO {
  pub fn get_instance_availabilities(
    &self,
    is_spot: bool,
    location: &LocationCode,
  ) -> InstancesAvailabilities {
    let params = vec![
      ("isSpot", if is_spot { "true" } else { "false" }),
      ("locationCode", location.into()),
    ];
    match self
      .query(&QueryParams {
        path,
        params,
        ..Default::default()
      })
      .into_json()
    {
      Ok(instance_availabilities) => instance_availabilities,
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }

  pub fn get_instance_availability_by_type(&self, instance_type: &str) -> bool {
    match self
      .query(&QueryParams {
        path: &format!("{path}/{instance_type}"),
        ..Default::default()
      })
      .into_json()
    {
      Ok(instance_availabilities) => instance_availabilities,
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }
}
