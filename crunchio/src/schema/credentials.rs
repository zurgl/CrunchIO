use serde::ser::SerializeStruct;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Credentials {
  client_id: String,
  client_secret: String,
}

impl Credentials {
  pub(crate) fn new(id: &str, secret: &str) -> Self {
    Self {
      client_id: id.to_owned(),
      client_secret: secret.to_owned(),
    }
  }
}

impl serde::Serialize for Credentials {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut credentials = serializer.serialize_struct("Credentials", 3)?;
    credentials.serialize_field("grant_type", "client_credentials")?;
    credentials.serialize_field("client_id", &self.client_id)?;
    credentials.serialize_field("client_secret", &self.client_secret)?;
    credentials.end()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_credentials_serializer() {
    let serialized_creds = serde_json::json!(Credentials::new(
      env!("CLIENT_ID"),
      env!("CLIENT_SECRET")
    ))
    .to_string();
    assert!(serialized_creds.contains("client_credentials"));
    assert!(serialized_creds.contains("grant_type"));
  }
}
