use crate::{Error, HttpRequestOptions, Result};

mod session;
use session::ASyncSession;

pub struct Client {
  pub(crate) client: reqwest::Client,
  pub(crate) session: ASyncSession,
  pub(crate) base_url: Option<url::Url>,
}

impl Client {
  pub async fn new(id: &str, secret: &str) -> Result<Self> {
    let mut headers = http::HeaderMap::new();
    headers.insert(
      "Content-Type",
      reqwest::header::HeaderValue::from_static("application/json"),
    );

    headers.insert(
      "Accept",
      reqwest::header::HeaderValue::from_static("application/json"),
    );

    let client = reqwest::ClientBuilder::new()
      .timeout(std::time::Duration::from_secs(5))
      .default_headers(headers)
      .build()
      .map_err(Error::RequestLib)?;

    let session = ASyncSession::authenticate(id, secret, &client).await?;

    Ok(Self {
      client,
      session,
      base_url: None,
    })
  }

  #[allow(dead_code)]
  pub(crate) async fn refresh_session(&mut self) -> Result<()> {
    self.session.refresh(&self.client).await
  }

  async fn http_request(
    &self,
    options: &HttpRequestOptions<'_>,
  ) -> Result<reqwest::Response> {
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

    let mut request_builder = self.client.request(method.clone(), url.clone());

    if !*is_public {
      request_builder =
        request_builder.header("Authorization", &self.session.inner.bearer());
    }

    if !params.is_empty() {
      request_builder = request_builder.query(&params);
    }

    if !body.is_null() {
      request_builder = request_builder.json(&body);
    }

    let request = request_builder.build().unwrap();

    self
      .client
      .execute(request)
      .await
      .map_err(Error::RequestLib)
  }
}

pub mod balance;
pub mod ssh_keys;
