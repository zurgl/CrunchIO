use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum VolumeType {
    NVMe,
    HDD,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Volume {
    id: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    instance_id: String,
    status: String,
    name: String,
    size: u32,
    is_os_volume: bool,
    created_at: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    target: String,
    #[serde(rename(deserialize = "type"))]
    volume_type: String,
    location: String,
    ssh_key_ids: Vec<String>,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

pub type Volumes = Vec<Volume>;
