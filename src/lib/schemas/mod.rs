pub mod instance;
pub use instance::Instance;

pub mod balance;
pub use balance::Balance;

pub mod scripts;
pub use scripts::Scripts;

pub mod images;
pub use images::Images;

pub mod ssh_keys;
pub use ssh_keys::SshKeys;

pub mod volumes;
pub use volumes::Volumes;

pub mod locations;
pub use locations::{Location, Locations};

pub type Instances = Vec<Instance>;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct InstancesAvailabilitie {
    location_code: String,
    availabilities: Vec<String>,
}

pub type InstancesAvailabilities = Vec<InstancesAvailabilitie>;

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub struct Script {
//     id: String,
//     name: String,
//     script: String,
// }

// pub type Scripts = Vec<Script>;

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub enum VolumeType {
//     NVMe,
//     HDD,
// }

// pub type VolumeIds = String;

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub enum Currency {
//     USD,
// }

// #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
// pub enum ActionType {
//     Start,
//     Shutdown,
//     Restore,
//     Delete,
//     Hibernate,
// }
