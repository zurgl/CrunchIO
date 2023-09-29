use crate::{utils::deserialize_null_default, Method};

use super::{CrunchIO, QueryParams};
use serde::{Deserialize, Serialize};
use ureq::json;
// use uuid::Uuid;

impl CrunchIO {
    pub fn deploy_new_instance(&self) -> String {
        let ssh_key_ids = "f42ab232-eb48-485f-b188-d528ebbf1beb";
        let hostname = "cuda";
        let description = "Rust cuda gpu server";
        let location_code = "FIN-01";
        let image = "9eb0f166-f119-49c9-ba55-e857dd055500";
        let instance_type = "1V100.6V";
        let is_spot = true;

        let payload = ureq::json!({
          "instance_type": instance_type,
          "image": image,
          "ssh_key_ids": ssh_key_ids,
          "hostname": hostname,
          "description": description,
          "location_code": location_code,
          "is_spot": is_spot
        });

        match self
            .query(&QueryParams {
                path: PATH,
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
