// src/config/loader.rs
use crate::config::types::Config;
use crate::constants::CONFIG_FILE;
use serde_json;
use std::fs;
use std::error::Error;

pub fn load(path: &str) -> Result<Config, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn save(config: &Config, path: &str) -> Result<(), Box<dyn Error>> {
    let content = serde_json::to_string_pretty(config)?;
    fs::write(path, content)?;
    Ok(())
}

pub fn exists() -> bool {
    fs::metadata(CONFIG_FILE).is_ok()
}
