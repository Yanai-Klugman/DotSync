use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
    pub global: GlobalConfig,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Profile {
    pub dotfiles: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GlobalConfig {
    pub default_profile: String,
}
