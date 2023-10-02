use super::{CrunchIO, QueryParams};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
  id: Uuid,
  details: Vec<String>,
  image_type: String,
  name: String,
}

pub type Images = Vec<Image>;

const PATH: &str = "images";

impl CrunchIO {
  pub fn get_all_images_types(&self) -> Images {
    match self
      .query(&QueryParams {
        path: PATH,
        ..Default::default()
      })
      .into_json()
    {
      Ok(images) => images,
      Err(error) => panic!("Json parsing failed with: {error}"),
    }
  }
}
