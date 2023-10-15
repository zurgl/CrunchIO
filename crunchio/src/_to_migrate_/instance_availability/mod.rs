// use super::{locations::LocationCode, CrunchIO, Error, QueryParams, Result};

// use super::routes::INSTANCE_AVAILABILITY as path;

// impl CrunchIO {
//   pub fn get_instance_availabilities(
//     &self,
//     is_spot: bool,
//     location: &LocationCode,
//   ) -> Result<Vec<Availability>> {
//     let params = vec![
//       ("isSpot", if is_spot { "true" } else { "false" }),
//       ("locationCode", location.into()),
//     ];
//     self
//       .http_request(&QueryParams {
//         path,
//         params,
//         ..Default::default()
//       })?
//       .into_json()
//       .map_err(Error::JsonParsing)
//   }

//   pub fn get_instance_availability_by_type(&self, instance_type: &str) -> Result<bool> {
//     self
//       .http_request(&QueryParams {
//         path: &format!("{path}/{instance_type}"),
//         ..Default::default()
//       })?
//       .into_json()
//       .map_err(Error::JsonParsing)
//   }
// }
