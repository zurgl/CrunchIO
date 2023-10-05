use super::{CrunchIO, Error, QueryParams, Result};
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

use super::routes::IMAGES as path;

impl CrunchIO {
  pub fn get_all_images_types(&self) -> Result<Images> {
    self
      .query(&QueryParams {
        path,
        ..Default::default()
      })
      .into_json()
      .map_err(Error::JsonParsing)
  }
}
