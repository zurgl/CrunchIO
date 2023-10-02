use super::{CrunchIO, Method, QueryParams};
use serde::{Deserialize, Serialize};
use ureq::json;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
  pub id: Uuid,
  pub name: String,
  pub script: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeletedScripts {
  pub count: u64,
}

pub type Scripts = Vec<Script>;

const PATH: &str = "scripts";

impl CrunchIO {
  // Private require Bearer token
  pub fn get_all_startup_scripts(&self) -> Scripts {
    match self
      .query(&QueryParams {
        path: PATH,
        ..Default::default()
      })
      .into_json()
    {
      Ok(scripts) => scripts,
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }

  // Private require Bearer token
  pub fn add_startup_script(&self, name: &str, script: &str) -> Uuid {
    let payload = json!({
      "name": name,
      "script": script
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
  pub fn delete_startup_scripts(&self, scripts: &Vec<Uuid>) -> DeletedScripts {
    let payload = json!({
      "scripts": scripts
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
  pub fn get_startup_script_by_id(&self, id: &Uuid) -> Scripts {
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
  pub fn delete_startup_script_by_id(&self, id: &Uuid) -> DeletedScripts {
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
