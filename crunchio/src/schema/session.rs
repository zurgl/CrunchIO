#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Session {
  pub(crate) access_token: String,
  pub(crate) expires_in: u64,
  pub(crate) refresh_token: String,
  pub(crate) token_type: String,
  pub(crate) scope: String,
  #[serde(default = "std::time::SystemTime::now")]
  pub(crate) updated_at: std::time::SystemTime,
}

impl Session {
  pub fn bearer(&self) -> String {
    let token = self.access_token.clone();
    format!("Bearer {token}")
  }
  pub fn is_deprecated(&self) -> bool {
    let now = std::time::SystemTime::now();
    let elapsed = now.duration_since(self.updated_at).unwrap();
    elapsed.as_secs() >= self.expires_in
  }
}
