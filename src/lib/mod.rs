pub mod constants;
use constants::{api, routes};

pub mod error;

pub mod ionos;

pub mod schemas;
use schemas::instance::RunningInstance;

pub mod credentials;
use credentials::Credentials;

pub mod session;
use session::Session;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use ureq::{json, AgentBuilder, Error, Response};
use url::Url;

pub struct CrunchIO {
    pub client: ureq::Agent,
    pub credentials: Credentials,
    pub session: Session,
    pub base_url: Url,
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct QueryParams<'a> {
    method: Method,
    path: &'a str,
    payload: Option<Value>,
    params: Option<&'a str>,
}

impl Default for CrunchIO {
    fn default() -> Self {
        let credentials = Credentials::default();
        let client = AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();

        let session = Session::set_tokens(&client, &credentials);

        let base_url = Url::parse(api::URL).unwrap();
        println!("{base_url:?}");

        Self {
            client,
            credentials,
            session,
            base_url,
        }
    }
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

impl CrunchIO {
    fn query(&self, params: &QueryParams) -> Response {
        let QueryParams {
            method,
            path,
            payload,
            ..
        } = params;

        let url = &mut self.base_url.clone();
        url.set_path(&format!("{api_version}/{path}", api_version = api::VERSION));

        let request = self.client.request_url(method.into(), url);
        let request = request.set("Content-Type", "application/json");
        let request = request.set("Authorization", &self.session.bearer());
        let request = request.set("Accept", "application/json");

        match {
            if let Some(payload) = payload {
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

    pub fn get_all_instance_types(&self) -> schemas::Instances {
        match self
            .query(&QueryParams {
                path: routes::INSTANCE_TYPES,
                ..Default::default()
            })
            .into_json()
        {
            Ok(instances) => instances,
            Err(error) => panic!("Json parsing failed with: {error}"),
        }
    }

    pub fn get_instances(&self) -> schemas::RunningInstances {
        match self
            .query(&QueryParams {
                path: routes::INSTANCES,
                ..Default::default()
            })
            .into_json()
        {
            Ok(instances) => instances,
            Err(error) => panic!("Json parsing failed with: {error}"),
        }
    }

    pub fn shutdown(&self, id: &str) -> String {
        let response = self
            .client
            .put(routes::INSTANCES)
            .set("Content-Type", "application/json")
            .set("Authorization", &self.session.bearer())
            .send_json(ureq::json!({
              "id": id,
              "action": "shutdown"
            }))
            .expect("cannot post the req");
        response.into_string().unwrap()
    }

    pub fn start(&self, id: &str) -> String {
        let response = self
            .client
            .put(routes::INSTANCES)
            .set("Content-Type", "application/json")
            .set("Authorization", &self.session.bearer())
            .send_json(ureq::json!({
              "id": id,
              "action": "start"
            }))
            .expect("cannot post the req");
        response.into_string().unwrap()
    }

    pub fn hibernate(&self, id: &str) -> String {
        self.shutdown(id);

        let response = self
            .client
            .put(routes::INSTANCES)
            .set("Content-Type", "application/json")
            .set("Authorization", &self.session.bearer())
            .send_json(ureq::json!({
              "id": id,
              "action": "hibernate"
            }))
            .expect("cannot post the req");
        response.into_string().unwrap()
    }

    pub fn deploy_new_instance(&self) -> String {
        let ssh_key_ids = "f42ab232-eb48-485f-b188-d528ebbf1beb";
        let hostname = "cuda";
        let description = "Rust cuda gpu server";
        let location_code = "FIN-01";
        let image = "9eb0f166-f119-49c9-ba55-e857dd055500";
        let instance_type = "1V100.6V";
        let is_spot = true;

        let response = self
            .client
            .post(routes::INSTANCES)
            .set("Content-Type", "application/json")
            .set("'Accept", "application/json")
            .set("Authorization", &self.session.bearer())
            .send_json(ureq::json!({
              "instance_type": instance_type,
              "image": image,
              "ssh_key_ids": ssh_key_ids,
              "hostname": hostname,
              "description": description,
              "location_code": location_code,
              "is_spot": is_spot
            }))
            .expect("cannot post the req");
        response.into_string().unwrap()
    }

    pub fn get_instance_by_id(&self, id: &str) -> RunningInstance {
        match self
            .query(&QueryParams {
                path: &format!("{}/{}", routes::INSTANCES, id),
                ..Default::default()
            })
            .into_json()
        {
            Ok(locations) => locations,
            Err(error) => panic!("Json parsing failed with: {error}"),
        }
    }

    pub fn get_ssh_keys(&self) -> schemas::SshKeys {
        match self
            .query(&QueryParams {
                path: routes::SSH_KEYS,
                ..Default::default()
            })
            .into_json()
        {
            Ok(ssh_keys) => ssh_keys,
            Err(error) => panic!("Json parsing failed with: {error}"),
        }
    }

    pub fn get_scripts(&self) -> schemas::Scripts {
        match self
            .query(&QueryParams {
                path: routes::SCRIPTS,
                ..Default::default()
            })
            .into_json()
        {
            Ok(scripts) => scripts,
            Err(error) => panic!("Json parsing failed with: {error}"),
        }
    }

    pub fn get_instance_availabilities(&self) -> schemas::InstancesAvailabilities {
        match self
            .query(&QueryParams {
                path: routes::INSTANCE_AVAILABILITY,
                ..Default::default()
            })
            .into_json()
        {
            Ok(instance_availabilities) => instance_availabilities,
            Err(error) => panic!("Json parsing failed with: {error}"),
        }
    }

    pub fn get_images(&self) -> schemas::Images {
        match self
            .query(&QueryParams {
                path: routes::IMAGES,
                ..Default::default()
            })
            .into_json()
        {
            Ok(images) => images,
            Err(error) => panic!("Json parsing failed with: {error}"),
        }
    }

    pub fn get_volumes(&self) -> schemas::Volumes {
        match self
            .query(&QueryParams {
                path: routes::VOLUMES,
                ..Default::default()
            })
            .into_json()
        {
            Ok(volumes) => volumes,
            Err(error) => panic!("Json parsing failed with: {error}"),
        }
    }

    pub fn get_locations(&self) -> schemas::Locations {
        match self
            .query(&QueryParams {
                path: routes::LOCATIONS,
                ..Default::default()
            })
            .into_json()
        {
            Ok(locations) => locations,
            Err(error) => panic!("Json parsing failed with: {error}"),
        }
    }

    pub fn get_balance(&self) -> schemas::Balance {
        match self
            .query(&QueryParams {
                path: routes::BALANCE,
                ..Default::default()
            })
            .into_json()
        {
            Ok(balance) => balance,
            Err(error) => panic!("Json parsing failed with: {error}"),
        }
    }
}
