// src/config/types.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Profile {
    pub dotfiles: Vec<Dotfile>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Dotfile {
    pub source: String,
    pub destination: String,
}
