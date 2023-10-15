// use super::{CpuInfo, CrunchIO, Error, GpuInfo, Memory, QueryParams, Result};
// use super::routes::INSTANCE_TYPES as path;

// impl CrunchIO {
//   pub fn get_all_instance_types(&self) -> Result<Vec<InstanceType>> {
//     self
//       .public_http_request(&QueryParams {
//         path,
//         ..Default::default()
//       })?
//       .into_json()
//       .map_err(Error::JsonParsing)
//   }
// }
