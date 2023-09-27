use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
    id: String,
    name: String,
    script: String,
}

pub type Scripts = Vec<Script>;
