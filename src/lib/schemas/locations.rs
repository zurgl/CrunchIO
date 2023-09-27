use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    code: String,
    name: String,
    country_code: String,
}

pub type Locations = Vec<Location>;
