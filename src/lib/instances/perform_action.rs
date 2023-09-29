use crate::{utils::deserialize_null_default, Method};

use super::{CrunchIO, QueryParams};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionType {
    Start,
    Shutdown,
    Restore,
    Delete,
    Hibernate,
}

impl CrunchIO {
    pub fn perform_action_on_instance(&self, id: &str, action: ActionType) -> String {
        let payload = json!({
          "id": id,
          "action": action
        });
        match self
            .query(&QueryParams {
                path: super::PATH,
                method: Method::PUT,
                payload,
                ..Default::default()
            })
            .into_json()
        {
            Ok(info) => info,
            Err(error) => panic!("Json parsing failed with: {error}"),
        }
    }
}
