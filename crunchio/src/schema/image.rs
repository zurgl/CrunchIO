#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Image {
  id: uuid::Uuid,
  details: Vec<String>,
  image_type: String,
  name: String,
}
