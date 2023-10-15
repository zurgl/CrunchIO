use crate::{
  async_request, routes,
  schema::{DeletedKeys, SshAddKeyBody, SshKey},
  Error, HttpRequestOptions, Result,
};

impl super::Client {
  pub async fn get_all_ssh_keys(&self) -> Result<Vec<SshKey>> {
    async_request!(
      self,
      &HttpRequestOptions {
        route: vec![routes::SSH_KEYS],
        ..Default::default()
      }
    )
  }

  pub async fn add_ssh_keys(&self, body: &SshAddKeyBody) -> Result<uuid::Uuid> {
    let response: String = async_request!(
      self,
      &HttpRequestOptions {
        route: vec![routes::SSH_KEYS],
        method: http::Method::POST,
        body: serde_json::json!(body),
        ..Default::default()
      }
    )?;

    uuid::Uuid::try_parse(&response).map_err(Error::UuidParsing)
  }

  pub async fn delete_ssh_keys(
    &self,
    keys: &Vec<uuid::Uuid>,
  ) -> Result<DeletedKeys> {
    async_request!(
      self,
      &HttpRequestOptions {
        route: vec![routes::SSH_KEYS],
        method: http::Method::DELETE,
        body: serde_json::json!({
          "keys": keys
        }),
        ..Default::default()
      }
    )
  }

  pub async fn get_ssh_key_by_id(
    &self,
    id: &uuid::Uuid,
  ) -> Result<Vec<SshKey>> {
    async_request!(
      self,
      &HttpRequestOptions {
        route: vec![routes::SSH_KEYS, id.to_string().as_str()],
        ..Default::default()
      }
    )
  }

  pub async fn delete_ssh_key_by_id(
    &self,
    id: &uuid::Uuid,
  ) -> Result<DeletedKeys> {
    async_request!(
      self,
      &HttpRequestOptions {
        route: vec![routes::SSH_KEYS, id.to_string().as_str()],
        method: http::Method::DELETE,
        ..Default::default()
      }
    )
  }
}
