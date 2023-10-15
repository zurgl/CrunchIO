use crate::{request, routes, schema::UserBalance, HttpRequestOptions, Result};

impl super::Client {
  pub fn get_user_balance(&self) -> Result<UserBalance> {
    request!(
      self,
      &HttpRequestOptions {
        route: vec![routes::BALANCE],
        ..Default::default()
      }
    )
  }
}
