use crate::config::loader;
use crate::constants;
use crate::config::types::{Config, Profile};
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn create_profile(profile_name: &str) -> Result<(), Box<dyn Error>> {
    let mut config = loader::load(Path::new(constants::CONFIG_FILE)).unwrap_or_default();
    let profile = Profile::default();
    config.profiles.insert(profile_name.to_string(), profile);
    save_config(&config)?;
    Ok(())
}

pub fn list_profiles() -> Result<(), Box<dyn Error>> {
    let config = loader::load(Path::new(constants::CONFIG_FILE)).unwrap_or_default();
    for profile in config.profiles.keys() {
        println!("{}", profile);
    }
    Ok(())
}

pub fn switch_profile(profile_name: &str) -> Result<(), Box<dyn Error>> {
    let mut config = loader::load(Path::new(constants::CONFIG_FILE)).unwrap_or_default();
    if !config.profiles.contains_key(profile_name) {
        return Err(format!("Profile '{}' not found", profile_name).into());
    }
    let default_profile = config.profiles.get(profile_name).unwrap().clone();
    config.profiles.insert("default".to_string(), default_profile);
    save_config(&config)?;
    Ok(())
}

fn save_config(config: &Config) -> Result<(), Box<dyn Error>> {
    let config_content = toml::to_string_pretty(config)?;
    fs::write(constants::CONFIG_FILE, config_content)?;
    Ok(())
}
