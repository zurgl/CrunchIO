pub mod types;

use crate::utils::deserialize_null_default;

use super::{CrunchIO, QueryParams};
use serde::{Deserialize, Serialize};
// use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CpuInfo {
    description: String,
    number_of_cores: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GpuInfo {
    description: String,
    number_of_gpus: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Memory {
    description: String,
    size_in_gigabytes: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Instance {
    gpu: GpuInfo,
    id: String,
    instance_type: String,
    memory: Memory,
    gpu_memory: Memory,
    model: String,
    name: String,
    price_per_hour: String,
    spot_price: String,
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
    id: String,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstancesAvailabilitie {
    location_code: String,
    availabilities: Vec<String>,
}

pub type InstancesAvailabilities = Vec<InstancesAvailabilitie>;

pub type RunningInstances = Vec<RunningInstance>;

pub type Instances = Vec<Instance>;

const PATH: &str = "instances";

impl CrunchIO {
    pub fn get_instances(&self) -> RunningInstances {
        match self
            .query(&QueryParams {
                path: PATH,
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
                path: &format!("{}/{}", PATH, id),
                ..Default::default()
            })
            .into_json()
        {
            Ok(locations) => locations,
            Err(error) => panic!("Json parsing failed with: {error}"),
        }
    }

    pub fn get_instance_availabilities(&self) -> InstancesAvailabilities {
        match self
            .query(&QueryParams {
                path: PATH,
                ..Default::default()
            })
            .into_json()
        {
            Ok(instance_availabilities) => instance_availabilities,
            Err(error) => panic!("Json parsing failed with: {error}"),
        }
    }
}
