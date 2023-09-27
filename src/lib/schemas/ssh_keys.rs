use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SshKey {
    id: String,
    name: String,
    key: String,
}

pub type SshKeys = Vec<SshKey>;
