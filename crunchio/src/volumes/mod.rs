use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{
  locations::LocationCode, CrunchIO, Error, Method, QueryParams, Result,
  _shared_::deserialize_null_default,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum VolumeType {
  #[default]
  NVMe,
  HDD,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Volume {
  id: Uuid,
  #[serde(deserialize_with = "deserialize_null_default")]
  instance_id: Uuid,
  status: VolumeStatus,
  name: String,
  size: u64,
  is_os_volume: bool,
  created_at: String,
  #[serde(deserialize_with = "deserialize_null_default")]
  target: String,
  #[serde(rename(deserialize = "type"))]
  volume_type: VolumeType,
  location: LocationCode,
  ssh_key_ids: Vec<Uuid>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum VolumeAction {
  #[default]
  Attach,
  Detach,
  Delete,
  Rename,
  #[serde(rename(deserialize = "increase-size"))]
  IncreaseSize,
  Restore,
  Clone,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VolumeActionBody {
  id: Uuid,
  action: VolumeAction,
  size: u64,
  name: String,
  instance_id: Uuid,
  #[serde(rename(deserialize = "type"))]
  volume_type: Option<VolumeType>,
}

pub type Volumes = Vec<Volume>;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct VolumeCreateBody<'a> {
  name: &'a str,
  size: u64,
  #[serde(rename = "type")]
  volume_type: VolumeType,
  location_code: LocationCode,
  #[serde(skip_serializing_if = "Option::is_none")]
  instance_id: Option<Uuid>,
}

use super::routes::VOLUMES as path;

impl CrunchIO {
  pub fn get_all_volumes(&self) -> Result<Volumes> {
    self
      .http_request(&QueryParams {
        path,
        ..Default::default()
      })?
      .into_json()
      .map_err(Error::JsonParsing)
  }

  pub fn perform_action_on_volume(&self) -> Result<Volumes> {
    self
      .http_request(&QueryParams {
        path,
        ..Default::default()
      })?
      .into_json()
      .map_err(Error::JsonParsing)
  }

  pub fn create_new_volume(&self, name: &str, size: u64) -> Result<String> {
    assert!(name.len() <= 60);
    assert!(size >= 40);
    let payload = json!(VolumeCreateBody {
      name,
      size,
      ..Default::default()
    });

    self
      .http_request(&QueryParams {
        path,
        payload,
        method: Method::POST,
        ..Default::default()
      })?
      .into_string()
      .map_err(Error::JsonParsing)
  }

  pub fn delete_volume_by_id(&self, id: &str) -> Result<String> {
    self
      .http_request(&QueryParams {
        path: &format!("{path}/{id}"),
        method: Method::DELETE,
        ..Default::default()
      })?
      .into_string()
      .map_err(Error::JsonParsing)
  }

  pub fn get_volume_by_id(&self, id: &str) -> Result<String> {
    self
      .http_request(&QueryParams {
        path: &format!("{path}/{id}"),
        ..Default::default()
      })?
      .into_string()
      .map_err(Error::JsonParsing)
  }
}
