pub mod balance;
pub mod error;
pub mod images;
pub mod instance_availability;
pub mod instance_types;
pub mod instances;
pub use instances::InstanceCreateBody;

pub mod locations;
pub mod ssh_keys;
pub mod startup_scripts;
pub mod volume_types;
pub mod volumes;

pub mod _shared_;
use _shared_::*;

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
    self.query_(query_params, true)
  }

  fn public_query(&self, query_params: &QueryParams) -> Response {
    self.query_(query_params, false)
  }

  pub fn refresh_session(&mut self) {
    self.session.refresh(&self.client)
  }

  fn query_(&self, query_params: &QueryParams, is_private: bool) -> Response {
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
    if is_private {
      request = request.set("Authorization", &self.session.bearer());
    }
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
