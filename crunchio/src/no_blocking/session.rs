use crate::{
  routes,
  schema::{Credentials, Session},
  Error, Result,
};

pub struct ASyncSession {
  pub(super) inner: Session,
}

impl ASyncSession {
  pub(crate) async fn authenticate(
    id: &str,
    secret: &str,
    client: &reqwest::Client,
  ) -> Result<Self> {
    client
      .post(routes::AUTHENTICATION)
      .json(&Credentials::new(id, secret))
      .send()
      .await
      .map_err(Error::RequestLib)?
      .json()
      .await
      .map(|session| Self { inner: session })
      .map_err(Error::RequestLib)
  }

  pub(crate) async fn refresh(
    &mut self,
    client: &reqwest::Client,
  ) -> Result<()> {
    client
      .post(routes::AUTHENTICATION)
      .json(&self.inner)
      .send()
      .await
      .map_err(Error::RequestLib)?
      .json()
      .await
      .map(|session| self.inner = session)
      .map_err(Error::RequestLib)
  }
}
