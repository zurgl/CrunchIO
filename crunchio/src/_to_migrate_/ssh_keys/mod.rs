// use super::{CrunchIO, Error, Method, QueryParams, Result};
// use serde::{Deserialize, Serialize};
// use ureq::json;
// use uuid::Uuid;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct DeletedKeys {
//   pub count: u64,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct SshKey {
//   pub id: Uuid,
//   pub name: String,
//   pub key: String,
// }

// pub type SshKeys = Vec<SshKey>;

// use super::routes::SSH_KEYS as path;

// impl CrunchIO {
//   // Private require Bearer token
//   pub fn get_all_ssh_keys(&self) -> Result<SshKeys> {
//     self
//       .http_request(&QueryParams {
//         path,
//         ..Default::default()
//       })?
//       .into_json()
//       .map_err(Error::JsonParsing)
//   }

//   // Private require Bearer token
//   pub fn add_ssh_keys(&self, name: &str, key: &str) -> Result<Uuid> {
//     let payload = json!({
//       "name": name,
//       "key": key
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
//       .map(|value| Uuid::try_parse(&value).unwrap())
//   }

//   // Private require Bearer token
//   pub fn delete_ssh_keys(&self, keys: &Vec<Uuid>) -> Result<DeletedKeys> {
//     let payload = json!({
//       "keys": keys
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
//   pub fn get_ssh_key_by_id(&self, id: &Uuid) -> Result<SshKeys> {
//     self
//       .http_request(&QueryParams {
//         path: &format!("{path}/{id}"),
//         ..Default::default()
//       })?
//       .into_json()
//       .map_err(Error::JsonParsing)
//   }

//   // Private require Bearer token
//   pub fn delete_ssh_key_by_id(&self, id: &Uuid) -> Result<DeletedKeys> {
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
