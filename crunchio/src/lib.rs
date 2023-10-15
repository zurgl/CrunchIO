#![allow(clippy::result_large_err)]

pub mod error;
pub use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) mod api {
  pub(crate) const URL: &str = "https://api.datacrunch.io";
  pub(crate) const VERSION: &str = "v1";
}

pub(crate) mod routes {
  pub(crate) const AUTHENTICATION: &str =
    "https://api.datacrunch.io/v1/oauth2/token";
  // pub(crate) const IMAGES: &str = "images";
  pub(crate) const BALANCE: &str = "balance";
  // pub(crate) const LOCATIONS: &str = "locations";
  // pub(crate) const INSTANCES: &str = "instances";
  // pub(crate) const INSTANCE_TYPES: &str = "instance-types";
  // pub(crate) const INSTANCE_AVAILABILITY: &str = "instance-availability";
  pub(crate) const SSH_KEYS: &str = "sshkeys";
  // pub(crate) const SCRIPTS: &str = "scripts";
  // pub(crate) const VOLUMES: &str = "volumes";
  // pub(crate) const VOLUME_TYPES: &str = "volume-types";
}

pub mod utils {
  use serde::{Deserialize, Deserializer};

  pub fn deserialize_null_default<'de, D, T>(
    deserializer: D,
  ) -> Result<T, D::Error>
  where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
  {
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
  }

  pub fn set_url_with_routes<I>(routes: I) -> super::Result<url::Url>
  where
    I: IntoIterator,
    I::Item: AsRef<str>,
  {
    let mut url =
      url::Url::parse(super::api::URL).map_err(super::Error::UrlParsing)?;
    url
      .path_segments_mut()
      .map_err(|_| super::Error::UrlMutation)?
      .push(super::api::VERSION)
      .extend(routes);

    Ok(url)
  }
}

#[derive(Clone, Default)]
pub(crate) struct HttpRequestOptions<'a> {
  pub(crate) method: http::Method,
  pub(crate) route: Vec<&'a str>,
  pub(crate) body: serde_json::Value,
  pub(crate) params: Vec<(&'a str, &'a str)>,
  pub(crate) is_public: bool,
}

pub mod blocking;

pub mod no_blocking;

#[macro_export]
macro_rules! async_request {
  ($self:expr, $options:expr) => {
    $self
      .http_request($options)
      .await?
      .json()
      .await
      .map_err($crate::error::Error::RequestLib)
  };
}

#[macro_export]
macro_rules! request {
  ($self:expr, $options:expr) => {
    $self
      .http_request($options)?
      .into_json()
      .map_err($crate::error::Error::JsonParing)
  };
}

pub mod schema;
pub use schema::{
  CpuInfo, Currency, DeletedKeys, GpuInfo, Memory, SshKey, UserBalance,
};
