use super::{CrunchIO, QueryParams};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub enum LocationCode {
  #[default]
  #[serde(rename = "FIN-01")]
  FIN01,
  #[serde(rename = "ICE-01")]
  ICE01,
}

impl From<&LocationCode> for &str {
  fn from(value: &LocationCode) -> Self {
    match value {
      LocationCode::FIN01 => "FIN-01",
      LocationCode::ICE01 => "ICE-01",
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
  pub code: LocationCode,
  pub name: String,
  pub country_code: String,
}

pub type Locations = Vec<Location>;

use super::routes::LOCATIONS as path;

impl CrunchIO {
  pub fn get_locations(&self) -> Locations {
    match self
      .query(&QueryParams {
        path,
        ..Default::default()
      })
      .into_json()
    {
      Ok(locations) => locations,
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }
}
