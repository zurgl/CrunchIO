// use super::{CrunchIO, Error, Method, QueryParams, Result};
// use serde::{Deserialize, Serialize};
// use ureq::json;
// use uuid::Uuid;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Script {
//   pub id: Uuid,
//   pub name: String,
//   pub script: String,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct DeletedScripts {
//   pub count: u64,
// }

// pub type Scripts = Vec<Script>;

// use super::routes::SCRIPTS as path;

// impl CrunchIO {
//   // Private require Bearer token
//   pub fn get_all_startup_scripts(&self) -> Result<Scripts> {
//     self
//       .http_request(&QueryParams {
//         path,
//         ..Default::default()
//       })?
//       .into_json()
//       .map_err(Error::JsonParsing)
//   }

//   // Private require Bearer token
//   pub fn add_startup_script(&self, name: &str, script: &str) -> Result<Uuid> {
//     let payload = json!({
//       "name": name,
//       "script": script
//     });
//     self
//       .http_request(&QueryParams {
//         path,
//         method: Method::POST,
//         payload,
//         ..Default::default()
//       })?
//       .into_string()
//       .map_err(Error::JsonParsing)
//       .and_then(|value| Uuid::try_parse(&value).map_err(Error::UuidParsing))
//   }

//   // Private require Bearer token
//   pub fn delete_startup_scripts(&self, scripts: &Vec<Uuid>) -> Result<DeletedScripts> {
//     let payload = json!({
//       "scripts": scripts
//     });
//     self
//       .http_request(&QueryParams {
//         path,
//         method: Method::DELETE,
//         payload,
//         ..Default::default()
//       })?
//       .into_json()
//       .map_err(Error::JsonParsing)
//   }

//   // Private require Bearer token
//   pub fn get_startup_script_by_id(&self, id: &Uuid) -> Result<Scripts> {
//     self
//       .http_request(&QueryParams {
//         path: &format!("{path}/{id}"),
//         ..Default::default()
//       })?
//       .into_json()
//       .map_err(Error::JsonParsing)
//   }

//   // Private require Bearer token
//   pub fn delete_startup_script_by_id(&self, id: &Uuid) -> Result<DeletedScripts> {
//     self
//       .http_request(&QueryParams {
//         path: &format!("{path}/{id}"),
//         method: Method::DELETE,
//         ..Default::default()
//       })?
//       .into_json()
//       .map_err(Error::JsonParsing)
//   }
// }
