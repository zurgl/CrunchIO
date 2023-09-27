pub mod api {
    pub const URL: &str = "https://api.datacrunch.io/v1";
    pub const VERSION: &str = "v1";
    pub const AUTHENTICATION: &str = "https://api.datacrunch.io/v1/oauth2/token";
}

pub mod routes {
    pub const IMAGES: &str = "images";
    pub const BALANCE: &str = "balance";
    pub const LOCATIONS: &str = "locations";
    pub const INSTANCES: &str = "instances";
    pub const INSTANCE_TYPES: &str = "instance-types";
    pub const INSTANCE_AVAILABILITY: &str = "instance-availability";
    pub const AUTHENTICATION: &str = "token";
    pub const SSH_KEYS: &str = "sshkeys";
    pub const SCRIPTS: &str = "scripts";
    pub const VOLUMES: &str = "volumes";
}
