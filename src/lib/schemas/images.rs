#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Image {
    id: String,
    details: Vec<String>,
    image_type: String,
    name: String,
}

pub type Images = Vec<Image>;
