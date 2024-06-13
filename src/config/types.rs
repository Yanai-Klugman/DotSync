// src/config/types.rs
use serde::{Deserialize, Serialize};

/// Configuration structure for DotSync.
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub global: GlobalConfig,
    pub profiles: Profiles,
}

/// Global configuration.
#[derive(Serialize, Deserialize)]
pub struct GlobalConfig {
    pub dependencies: Vec<String>,
}

/// Profiles configuration.
#[derive(Serialize, Deserialize)]
pub struct Profiles {
    pub personal: Profile,
}

/// Profile configuration.
#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub dotfiles: Vec<Dotfile>,
    pub dependencies: Vec<String>,
}

/// Dotfile configuration.
#[derive(Serialize, Deserialize)]
pub struct Dotfile {
    pub source: String,
    pub destination: String,
}
