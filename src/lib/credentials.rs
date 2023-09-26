#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Credentials {
    id: String,
    secret: String,
}

impl Default for Credentials {
    fn default() -> Self {
        Self {
            id: std::option_env!("CLIENT_ID").unwrap().to_string(),
            secret: std::option_env!("CLIENT_SECRET").unwrap().to_string(),
        }
    }
}

impl Credentials {
    pub fn as_payload(&self) -> serde_json::Value {
        ureq::json!({
            "grant_type": "client_credentials",
            "client_id": self.id,
            "client_secret": self.secret
        })
    }
}
