pub mod endpoints;
pub mod error;

pub mod schemas;

pub mod credentials;
use credentials::Credentials;

pub mod session;
use session::Session;

use std::time::Duration;
use ureq::AgentBuilder;

pub struct CrunchIO {
    pub client: ureq::Agent,
    pub credentials: Credentials,
    pub session: Session,
}

impl Default for CrunchIO {
    fn default() -> Self {
        let credentials = Credentials::default();
        let client = AgentBuilder::new()
            .timeout_read(Duration::from_secs(5))
            .timeout_write(Duration::from_secs(5))
            .build();

        let session = Session::set_tokens(&client, &credentials);

        Self {
            client,
            credentials,
            session,
        }
    }
}

impl CrunchIO {
    pub fn all_instance_types(&self) -> schemas::Instances {
        let response = self
            .client
            .get(endpoints::INSTANCE_TYPES)
            .set("Content-Type", "application/json")
            .set("Authorization", &self.session.bearer())
            .call()
            .unwrap();
        response.into_json().unwrap()
    }

    pub fn instances(&self) -> String {
        let response = self
            .client
            .get(endpoints::INSTANCES)
            .set("Content-Type", "application/json")
            .set("Authorization", &self.session.bearer())
            .call()
            .unwrap();
        response.into_string().unwrap()
    }

    pub fn deploy(&self) -> String {
        let ssh_key_ids = "f42ab232-eb48-485f-b188-d528ebbf1beb";
        let hostname = "cuda";
        let description = "Rust cuda gpu server";
        let location_code = "FIN-01";
        // let image = "ubuntu-22.04-cuda-12.0-docker";
        let instance_type = "1V100.6V";
        let volume_ids = vec!["9eb0f166-f119-49c9-ba55-e857dd055500"];

        let response = self
            .client
            .post(endpoints::INSTANCES)
            .set("Content-Type", "application/json")
            .set("'Accept", "application/json")
            .set("Authorization", &self.session.bearer())
            .send_json(ureq::json!({
              "instance_type": instance_type,
              "image": "",
              "ssh_key_ids": ssh_key_ids,
              "hostname": hostname,
              "description": description,
              "existing_volumes": volume_ids,
              "location_code": location_code
            }))
            .expect("cannot post the req");
        response.into_string().unwrap()
    }

    pub fn ssh_keys(&self) -> schemas::SshKeys {
        let response = self
            .client
            .get(endpoints::SSH_KEYS)
            .set("Content-Type", "application/json")
            .set("Authorization", &self.session.bearer())
            .call()
            .unwrap();
        response.into_json().unwrap()
    }

    pub fn scripts(&self) -> schemas::SshKeys {
        let response = self
            .client
            .get(endpoints::SCRIPTS)
            .set("Content-Type", "application/json")
            .set("Authorization", &self.session.bearer())
            .call()
            .unwrap();
        response.into_json().unwrap()
    }

    pub fn instance_availability(&self) -> schemas::InstancesAvailabilities {
        let response = self
            .client
            .get(endpoints::INSTANCE_AVAILABILITY)
            .set("Content-Type", "application/json")
            .set("Authorization", &self.session.bearer())
            .call()
            .unwrap();
        response.into_json().unwrap()
    }

    pub fn images(&self) -> schemas::Images {
        let response = self
            .client
            .get(endpoints::IMAGES)
            .set("Content-Type", "application/json")
            .set("Authorization", &self.session.bearer())
            .call()
            .unwrap();
        response.into_json().unwrap()
    }

    pub fn volumes(&self) -> schemas::Volumes {
        let response = self
            .client
            .get(endpoints::VOLUMES)
            .set("Content-Type", "application/json")
            .set("Authorization", &self.session.bearer())
            .call()
            .unwrap();
        response.into_json().unwrap()
    }

    pub fn locations(&self) -> schemas::Locations {
        let response = self
            .client
            .get(endpoints::LOCATIONS)
            .set("Content-Type", "application/json")
            .set("Authorization", &self.session.bearer())
            .call()
            .unwrap();
        response.into_json().unwrap()
    }

    pub fn balance(&self) -> schemas::Balance {
        let response = self
            .client
            .get(endpoints::BALANCE)
            .set("Content-Type", "application/json")
            .set("Authorization", &self.session.bearer())
            .call()
            .unwrap();
        response.into_json().unwrap()
    }
}
