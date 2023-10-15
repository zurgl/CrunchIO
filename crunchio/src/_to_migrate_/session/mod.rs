// use crate::api;
// use std::env;

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub struct Credentials {
//   id: String,
//   secret: String,
// }

// impl Default for Credentials {
//   fn default() -> Self {
//     Self {
//       id: env!("CLIENT_ID").to_owned(),
//       secret: env!("CLIENT_SECRET").to_owned(),
//     }
//   }
// }

// impl Credentials {
//   pub fn as_payload(&self) -> serde_json::Value {
//     ureq::json!({
//         "grant_type": "client_credentials",
//         "client_id": self.id,
//         "client_secret": self.secret
//     })
//   }
// }

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub struct Session {
//   access_token: String,
//   expires_in: u64,
//   refresh_token: String,
//   token_type: String,
//   scope: String,
//   #[serde(default = "std::time::SystemTime::now")]
//   updated_at: std::time::SystemTime,
// }

// impl Session {
//   pub fn bearer(&self) -> String {
//     let token = self.access_token.clone();
//     format!("Bearer {token}")
//   }

//   fn as_payload(&self) -> serde_json::Value {
//     ureq::json!({
//       "grant_type": "refresh_token",
//       "refresh_token": self.refresh_token,
//     })
//   }

//   pub(crate) fn set_tokens(client: &ureq::Agent) -> Self {
//     let credentials = Credentials::default();

//     let response: ureq::Response = client
//       .post(api::AUTHENTICATION)
//       .set("Content-Type", "application/json")
//       .send_json(credentials.as_payload())
//       .expect("cannot set the tokens");

//     response.into_json().expect("cannot parse the response")
//   }

//   pub fn is_deprecated(&self) -> bool {
//     let now = std::time::SystemTime::now();
//     let elapsed = now.duration_since(self.updated_at).unwrap();
//     elapsed.as_secs() >= self.expires_in
//   }

//   pub(crate) fn refresh(&mut self, client: &ureq::Agent) {
//     if self.is_deprecated() {
//       let response: ureq::Response = client
//         .post(api::AUTHENTICATION)
//         .set("Content-Type", "application/json")
//         .send_json(self.as_payload())
//         .expect("cannot refresh the tokens");

//       let Session {
//         access_token,
//         expires_in,
//         refresh_token,
//         updated_at,
//         ..
//       } = response.into_json().expect("cannot parse the response");

//       self.access_token = access_token;
//       self.expires_in = expires_in;
//       self.refresh_token = refresh_token;
//       self.updated_at = updated_at;
//     }
//   }
// }
