/*
 - Constructor pattern 4:45
   - new
   - default
 - Non Consuming Builder? 13:50
*/

#![allow(dead_code)]

pub mod error {
  use thiserror::Error;
  #[derive(Debug, Error)]
  pub enum Error {
    #[error("{0}")]
    Static(&'static str),
  }
}

use error::Error;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Request {
  url: String,
  method: String,
  body: Option<String>,
  headers: Vec<(String, String)>,
}

#[derive(Debug, Default)]
pub struct RequestBuilder {
  url: Option<String>,
  method: Option<String>,
  body: Option<String>,
  headers: Vec<(String, String)>,
}

impl RequestBuilder {
  pub fn new() -> Self {
    RequestBuilder::default()
  }

  pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
    let _ = self.url.insert(url.into());
    self
  }

  pub fn method(&mut self, method: impl Into<String>) -> &mut Self {
    let _ = self.method.insert(method.into());
    self
  }

  pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
    let _ = self.body.insert(body.into());
    self
  }

  pub fn header(
    &mut self,
    name: impl Into<String>,
    value: impl Into<String>,
  ) -> &mut Self {
    self.headers.push((name.into(), value.into()));
    self
  }

  pub fn build(&self) -> Result<Request> {
    let Some(url) = self.url.as_ref() else {
      return Err(Error::Static("An URL must be provided"));
    };

    let method = self.method.clone().unwrap_or_else(|| "GET".to_string());

    Ok(Request {
      url: url.to_owned(),
      method,
      body: self.body.clone(),
      headers: self.headers.clone(),
    })
  }
}

fn main() {
  let request = RequestBuilder::new()
    .url("some_url")
    .method("GET")
    .body("some_body")
    .build();

  println!("{request:#?}")
}
