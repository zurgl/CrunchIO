use crate::{
  CpuInfo, GpuInfo, Memory,
  _shared_::{deserialize_null_default, Method},
  locations::LocationCode,
  volumes::Volume,
};

use super::{CrunchIO, QueryParams};
use serde::{Deserialize, Serialize};
use ureq::json;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ActionType {
  Start,
  Shutdown,
  Restore,
  Delete,
  Hibernate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
  Running,
  Provisioning,
  Offline,
  StartingHibernation,
  Hibernating,
  Restoring,
  Error,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Storage {
  description: String,
  size_in_gigabytes: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunningInstance {
  id: Uuid,
  #[serde(deserialize_with = "deserialize_null_default")]
  pub ip: String,
  status: String,
  created_at: String,
  cpu: CpuInfo,
  gpu: GpuInfo,
  gpu_memory: Memory,
  memory: Memory,
  storage: Storage,
  hostname: String,
  description: String,
  location: String,
  price_per_hour: f32,
  is_spot: bool,
  instance_type: String,
  image: String,
  os_name: String,
  ssh_key_ids: Vec<String>,
  os_volume_id: String,
  #[serde(deserialize_with = "deserialize_null_default")]
  startup_script_id: String,
  #[serde(deserialize_with = "deserialize_null_default")]
  jupyter_token: String,
}

pub type RunningInstances = Vec<RunningInstance>;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OsVolume {
  name: String,
  size: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct InstanceCreateBody<'a> {
  pub instance_type: &'a str,
  pub image: &'a str,
  pub ssh_key_ids: &'a str,
  pub location_code: LocationCode,
  pub hostname: &'a str,
  pub description: &'a str,
  pub is_spot: bool,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub startup_script_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub volumes: Option<Volume>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub os_volume: Option<OsVolume>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub coupon: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub existing_volumes: Option<Vec<String>>,
}

use super::routes::INSTANCES as path;

impl CrunchIO {
  pub fn get_instances(&self) -> RunningInstances {
    match self
      .query(&QueryParams {
        path,
        ..Default::default()
      })
      .into_json()
    {
      Ok(instances) => instances,
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }

  pub fn get_instance_by_id(&self, id: &str) -> RunningInstance {
    match self
      .query(&QueryParams {
        path: &format!("{path}/{id}"),
        ..Default::default()
      })
      .into_json()
    {
      Ok(locations) => locations,
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }

  pub fn create_new_instance(&self, body: &InstanceCreateBody) -> String {
    let payload = json!(body);

    match self
      .query(&QueryParams {
        path,
        method: Method::POST,
        payload,
        ..Default::default()
      })
      .into_string()
    {
      Ok(info) => info,
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }

  pub fn perform_action_on_instance(&self, id: &str, action: ActionType) -> String {
    let payload = json!({
      "id": id,
      "action": action
    });
    match self
      .query(&QueryParams {
        path,
        method: Method::PUT,
        payload,
        ..Default::default()
      })
      .into_string()
    {
      Ok(info) => info,
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }
}
