use serde::{Deserialize, Deserializer, Serialize};

pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
  T: Default + Deserialize<'de>,
  D: Deserializer<'de>,
{
  let opt = Option::deserialize(deserializer)?;
  Ok(opt.unwrap_or_default())
}

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

pub mod routes {
  pub const IMAGES: &str = "images";
  pub const BALANCE: &str = "balance";
  pub const LOCATIONS: &str = "locations";
  pub const INSTANCES: &str = "instances";
  pub const INSTANCE_TYPES: &str = "instance-types";
  pub const INSTANCE_AVAILABILITY: &str = "instance-availability";
  pub const AUTHENTICATION: &str = "token";
  pub const SSH_KEYS: &str = "sshkeys";
  pub const SCRIPTS: &str = "scripts";
  pub const VOLUMES: &str = "volumes";
  pub const VOLUME_TYPES: &str = "volumes-types";
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum Method {
  #[default]
  GET,
  PUT,
  POST,
  DELETE,
}

impl From<&Method> for &str {
  fn from(method: &Method) -> Self {
    match method {
      Method::GET => "GET",
      Method::PUT => "PUT",
      Method::POST => "POST",
      Method::DELETE => "DELETE",
    }
  }
}
