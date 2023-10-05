use super::{CrunchIO, Error, QueryParams, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Currency {
  #[serde(rename = "usd")]
  USD,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBalance {
  pub amount: f64,
  pub currency: Currency,
}

use super::routes::BALANCE as path;

impl CrunchIO {
  pub fn get_user_balance(&self) -> Result<UserBalance> {
    self
      .query2(&QueryParams {
        path,
        ..Default::default()
      })?
      .into_json()
      .map_err(Error::JsonParsing)
  }
}
