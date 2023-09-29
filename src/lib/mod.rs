pub mod balance;
pub mod error;
pub mod images;
pub mod instances;
pub mod locations;
pub mod ssh_keys;
pub mod startup_scripts;
pub mod utils;
pub mod volumes;

pub mod session;
use session::Session;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use ureq::{json, AgentBuilder, Error, Response};
use url::Url;

pub mod api {
    pub const URL: &str = "https://api.datacrunch.io/v1";
    pub const VERSION: &str = "v1";
    pub const AUTHENTICATION: &str = "https://api.datacrunch.io/v1/oauth2/token";
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

pub struct CrunchIO {
    pub client: ureq::Agent,
    pub session: Session,
    pub base_url: Url,
}

impl Default for CrunchIO {
    fn default() -> Self {
        let client = AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();

        let session = Session::set_tokens(&client);

        let base_url = Url::parse(api::URL).unwrap();

        Self {
            client,
            session,
            base_url,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct QueryParams<'a> {
    method: Method,
    path: &'a str,
    payload: Value,
    params: Vec<(&'a str, &'a str)>,
}

impl CrunchIO {
    fn query(&self, query_params: &QueryParams) -> Response {
        let QueryParams {
            method,
            path,
            payload,
            params,
        } = query_params;

        let url = &mut self.base_url.clone();
        url.set_path(&format!("{api_version}/{path}", api_version = api::VERSION));

        let mut request = self.client.request_url(method.into(), url);
        request = request.set("Content-Type", "application/json");
        request = request.set("Authorization", &self.session.bearer());
        request = request.set("Accept", "application/json");

        for (param, value) in params {
            request = request.query(param, value);
        }

        match {
            if !payload.is_null() {
                request.send_json(json!(payload))
            } else {
                request.call()
            }
        } {
            Ok(response) => response,
            Err(Error::Status(code, response)) => {
                panic!("\n\nexit with:\n\tcode: {code:#?}\n\treponse: {response:#?}\n\n")
            }
            Err(error) => {
                panic!("Unknown error {error:#?}")
            }
        }
    }
}

// pub fn shutdown(&self, id: &str) -> String {
//     let response = self
//         .client
//         .put(routes::INSTANCES)
//         .set("Content-Type", "application/json")
//         .set("Authorization", &self.session.bearer())
//         .send_json(ureq::json!({
//           "id": id,
//           "action": "shutdown"
//         }))
//         .expect("cannot post the req");
//     response.into_string().unwrap()
// }

// pub fn start(&self, id: &str) -> String {
//     let response = self
//         .client
//         .put(routes::INSTANCES)
//         .set("Content-Type", "application/json")
//         .set("Authorization", &self.session.bearer())
//         .send_json(ureq::json!({
//           "id": id,
//           "action": "start"
//         }))
//         .expect("cannot post the req");
//     response.into_string().unwrap()
// }

// pub fn hibernate(&self, id: &str) -> String {
//     self.shutdown(id);

//     let response = self
//         .client
//         .put(routes::INSTANCES)
//         .set("Content-Type", "application/json")
//         .set("Authorization", &self.session.bearer())
//         .send_json(ureq::json!({
//           "id": id,
//           "action": "hibernate"
//         }))
//         .expect("cannot post the req");
//     response.into_string().unwrap()
// }

// pub fn deploy_new_instance(&self) -> String {
//     let ssh_key_ids = "f42ab232-eb48-485f-b188-d528ebbf1beb";
//     let hostname = "cuda";
//     let description = "Rust cuda gpu server";
//     let location_code = "FIN-01";
//     let image = "9eb0f166-f119-49c9-ba55-e857dd055500";
//     let instance_type = "1V100.6V";
//     let is_spot = true;

//     let response = self
//         .client
//         .post(routes::INSTANCES)
//         .set("Content-Type", "application/json")
//         .set("'Accept", "application/json")
//         .set("Authorization", &self.session.bearer())
//         .send_json(ureq::json!({
//           "instance_type": instance_type,
//           "image": image,
//           "ssh_key_ids": ssh_key_ids,
//           "hostname": hostname,
//           "description": description,
//           "location_code": location_code,
//           "is_spot": is_spot
//         }))
//         .expect("cannot post the req");
//     response.into_string().unwrap()
// }
