use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use ureq::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
  id: String,
  secret: String,
}

impl Default for Credentials {
  fn default() -> Self {
    Self {
      id: env!("CLIENT_ID").to_owned(),
      secret: env!("CLIENT_SECRET").to_owned(),
    }
  }
}

impl Credentials {
  pub fn as_payload(&self) -> Value {
    json!({
        "grant_type": "client_credentials",
        "client_id": self.id,
        "client_secret": self.secret
    })
  }
}
