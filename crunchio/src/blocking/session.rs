use crate::{
  routes,
  schema::{Credentials, Session},
  Error, Result,
};

pub struct SyncSession {
  pub(super) inner: Session,
}

impl SyncSession {
  pub(crate) fn authenticate(
    id: &str,
    secret: &str,
    client: &ureq::Agent,
  ) -> Result<Self> {
    let response: ureq::Response = client
      .post(routes::AUTHENTICATION)
      .send_json(Credentials::new(id, secret))
      .map_err(Error::UreqLib)?;

    let session = response.into_json().map_err(Error::JsonParing)?;

    Ok(Self { inner: session })
  }

  pub(crate) fn refresh(&mut self, client: &ureq::Agent) -> Result<()> {
    if self.inner.is_deprecated() {
      let response: ureq::Response = client
        .post(routes::AUTHENTICATION)
        .set("Content-Type", "application/json")
        .send_json(self.inner.clone())
        .map_err(Error::UreqLib)?;

      self.inner = response.into_json().map_err(Error::JsonParing)?;
    }
    Ok(())
  }
}
