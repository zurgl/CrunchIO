#![allow(clippy::large_enum_variant)]

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("Lib reqwest error")]
  RequestLib(#[from] reqwest::Error),

  #[error("Lib ureq error")]
  UreqLib(#[from] ureq::Error),

  #[error("failed to parse the url")]
  UrlParsing(#[source] url::ParseError),

  #[error("failed to parse the json")]
  JsonParing(#[source] std::io::Error),

  #[error("failed to mutate url")]
  UrlMutation,

  #[error("failed to parse uuid")]
  UuidParsing(#[source] uuid::Error),
}
