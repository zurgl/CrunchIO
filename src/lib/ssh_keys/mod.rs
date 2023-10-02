use super::{CrunchIO, Method, QueryParams};
use serde::{Deserialize, Serialize};
use ureq::json;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedKeys {
  pub count: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SshKey {
  pub id: Uuid,
  pub name: String,
  pub key: String,
}

pub type SshKeys = Vec<SshKey>;

const PATH: &str = "sshkeys";

impl CrunchIO {
  // Private require Bearer token
  pub fn get_all_ssh_keys(&self) -> SshKeys {
    match self
      .query(&QueryParams {
        path: PATH,
        ..Default::default()
      })
      .into_json()
    {
      Ok(data) => data,
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }

  // Private require Bearer token
  pub fn add_ssh_keys(&self, name: &str, key: &str) -> Uuid {
    let payload = json!({
      "name": name,
      "key": key
    });
    match self
      .query(&QueryParams {
        path: PATH,
        method: Method::POST,
        payload,
        ..Default::default()
      })
      .into_string()
    {
      Ok(value) => Uuid::try_parse(&value).unwrap(),
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }

  // Private require Bearer token
  pub fn delete_ssh_keys(&self, keys: &Vec<Uuid>) -> DeletedKeys {
    let payload = json!({
      "keys": keys
    });
    match self
      .query(&QueryParams {
        path: PATH,
        method: Method::DELETE,
        payload,
        ..Default::default()
      })
      .into_json()
    {
      Ok(count) => count,
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }

  // Private require Bearer token
  pub fn get_ssh_key_by_id(&self, id: &Uuid) -> SshKeys {
    let path = &format!("{PATH}/{id}");

    match self
      .query(&QueryParams {
        path,
        ..Default::default()
      })
      .into_json()
    {
      Ok(scripts) => scripts,
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }

  // Private require Bearer token
  pub fn delete_ssh_key_by_id(&self, id: &Uuid) -> DeletedKeys {
    let path = &format!("{PATH}/{id}");

    match self
      .query(&QueryParams {
        path,
        method: Method::DELETE,
        ..Default::default()
      })
      .into_json()
    {
      Ok(info) => info,
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }
}
