// src/utils/validation.rs
use crate::config::types::Config;
use std::fs;

pub fn validate_config(config: &Config) -> Result<(), String> {
    for profile in &config.profiles.personal.dotfiles {
        if !fs::metadata(&profile.source).is_ok() {
            return Err(format!("Source file does not exist: {}", profile.source));
        }
        if !fs::metadata(&profile.destination).is_ok() {
            return Err(format!("Destination directory does not exist: {}", profile.destination));
        }
    }
    Ok(())
}
