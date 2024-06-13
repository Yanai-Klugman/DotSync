use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dotfile {
    pub source: String,
    pub destination: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub dotfiles: Vec<Dotfile>,
    pub dependencies: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profiles {
    pub profiles: HashMap<String, Profile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
    pub dependencies: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub global: GlobalConfig,
    pub profiles: HashMap<String, Profile>,
}
