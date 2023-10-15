#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SshKey {
  pub id: uuid::Uuid,
  pub name: String,
  pub key: String,
}
