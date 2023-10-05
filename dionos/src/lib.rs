use std::time::Duration;

use ureq::AgentBuilder;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct DNSRecord {
  name: String,
  #[serde(rename(deserialize = "rootName"))]
  root_name: String,
  #[serde(rename(deserialize = "type"))]
  record_type: String,
  content: String,
  #[serde(rename(deserialize = "changeDate"))]
  change_date: String,
  ttl: u32,
  disabled: bool,
  id: String,
}

pub struct Ionos {
  pub client: ureq::Agent,
  pub public: String,
  pub private: String,
}

impl Default for Ionos {
  fn default() -> Self {
    let client = AgentBuilder::new()
      .timeout_read(Duration::from_secs(5))
      .timeout_write(Duration::from_secs(5))
      .build();

    let public = std::option_env!("DNS_PUB")
      .expect("DNS env must be set")
      .to_string();
    let private = std::option_env!("DNS_PRV")
      .expect("DNS env must be set")
      .to_string();

    Self {
      client,
      public,
      private,
    }
  }
}

impl Ionos {
  fn x_api_key(&self) -> String {
    format!(
      "{public}.{private}",
      public = self.public,
      private = self.private
    )
  }

  fn url(&self) -> String {
    format!(
      "{url}/zones/{zone_id}/records/{record_id}",
      url = "https://api.hosting.ionos.com/dns/v1",
      zone_id = "d5c223c8-c114-11ed-b6d9-0a586444046c",
      record_id = "ef026a1d-d4c0-5b7e-d617-705df6c6bc71"
    )
  }

  pub fn update_ip(&self, ip: &str) {
    assert!(ip.parse::<std::net::Ipv4Addr>().is_ok());

    let response = self
      .client
      .put(&self.url())
      .set("Content-Type", "application/json")
      .set("X-API-Key", &self.x_api_key())
      .send_json(ureq::json!({
        "disabled": false,
        "content": &ip,
        "ttl": 3600,
        "prio": 0
      }))
      .expect("cannot update the record");

    if response.status() == 200 {
      println!("The record has been updated with the corresponding ip.");
    } else {
      eprintln!("Failed with status code: {}", response.status());
    }
  }
}
