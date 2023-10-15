mod session;
use session::SyncSession;

use crate::HttpRequestOptions;

pub struct Client {
  pub(crate) client: ureq::Agent,
  pub(crate) session: SyncSession,
  pub(crate) base_url: Option<url::Url>,
}

impl Client {
  pub fn new(id: &str, secret: &str) -> crate::Result<Self> {
    let client = ureq::AgentBuilder::new()
      .timeout(std::time::Duration::from_secs(5))
      .build();

    let session = SyncSession::authenticate(id, secret, &client)?;

    Ok(Self {
      client,
      session,
      base_url: None,
    })
  }

  #[allow(dead_code)]
  pub(crate) fn refresh_session(&mut self) -> crate::Result<()> {
    self.session.refresh(&self.client)
  }
  fn http_request(
    &self,
    options: &HttpRequestOptions,
  ) -> crate::Result<ureq::Response> {
    let HttpRequestOptions {
      method,
      route,
      body,
      params,
      is_public,
    } = options;

    let url = match self.base_url {
      None => crate::utils::set_url_with_routes(route)?,
      Some(_) => unimplemented!("Not yet supported"),
    };

    let mut request = self.client.request_url(method.as_str(), &url);

    request = request.set("Content-Type", "application/json");
    request = request.set("Accept", "application/json");

    if !*is_public {
      request = request.set("Authorization", &self.session.inner.bearer());
    }

    for (param, value) in params {
      request = request.query(param, value);
    }

    let result = if body.is_null() {
      request.call()
    } else {
      request.send_json(serde_json::json!(body))
    };

    Ok(result?)
  }
}

pub mod balance;
