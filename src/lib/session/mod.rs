use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::SystemTime;
use ureq::{json, Agent, Response};

use crate::api;

pub mod credentials;
use credentials::Credentials;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    access_token: String,
    expires_in: u64,
    refresh_token: String,
    token_type: String,
    scope: String,
    #[serde(default = "SystemTime::now")]
    updated_at: SystemTime,
}

impl Session {
    pub fn bearer(&self) -> String {
        let token = self.access_token.clone();
        format!("Bearer {token}")
    }

    fn as_payload(&self) -> Value {
        json!({
          "grant_type": "refresh_token",
          "refresh_token": self.refresh_token,
        })
    }

    pub fn set_tokens(client: &Agent) -> Self {
        let credentials = Credentials::default();

        let response: Response = client
            .post(api::AUTHENTICATION)
            .set("Content-Type", "application/json")
            .send_json(credentials.as_payload())
            .expect("cannot set the tokens");

        response.into_json().expect("cannot parse the response")
    }

    pub fn is_deprecated(&self) -> bool {
        let now = SystemTime::now();
        let elapsed = now.duration_since(self.updated_at).unwrap();
        elapsed.as_secs() >= self.expires_in
    }

    pub fn refresh(&mut self, client: Agent) {
        if self.is_deprecated() {
            let response: Response = client
                .post(api::AUTHENTICATION)
                .set("Content-Type", "application/json")
                .send_json(self.as_payload())
                .expect("cannot refresh the tokens");

            let Session {
                access_token,
                expires_in,
                refresh_token,
                updated_at,
                ..
            } = response.into_json().expect("cannot parse the response");

            self.access_token = access_token;
            self.expires_in = expires_in;
            self.refresh_token = refresh_token;
            self.updated_at = updated_at;
        }
    }
}
