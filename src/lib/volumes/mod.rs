use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{locations::LocationCode, utils::deserialize_null_default, CrunchIO, QueryParams};

pub mod types;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum VolumeType {
    #[default]
    NVMe,
    HDD,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub enum VolumeStatus {
    #[default]
    #[serde(rename = "attached")]
    Attached,
    #[serde(rename = "detached")]
    Detached,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "ordered")]
    Ordered,
    #[serde(rename = "creating")]
    Creating,
    #[serde(rename = "deleting")]
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

pub type Volumes = Vec<Volume>;

const PATH: &str = "volumes";

impl CrunchIO {
    pub fn get_all_storage_volumes(&self) -> Volumes {
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
