use super::{CrunchIO, QueryParams};
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

const PATH: &str = "balance";

impl CrunchIO {
    pub fn get_user_balance(&self) -> UserBalance {
        match self
            .query(&QueryParams {
                path: PATH,
                ..Default::default()
            })
            .into_json()
        {
            Ok(balance) => balance,
            Err(error) => panic!("Json parsing failed with: {error}"),
        }
    }
}
