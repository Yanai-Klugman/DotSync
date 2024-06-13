use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Config {
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Profile {
    pub dotfiles: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Dotfile {
    pub source: String,
    pub destination: String,
}
