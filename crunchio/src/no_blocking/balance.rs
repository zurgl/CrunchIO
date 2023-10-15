use crate::{
  async_request, routes, schema::UserBalance, HttpRequestOptions, Result,
};

impl super::Client {
  pub async fn get_user_balance(&self) -> Result<UserBalance> {
    async_request!(
      self,
      &HttpRequestOptions {
        route: vec![routes::BALANCE],
        ..Default::default()
      }
    )
  }
}
