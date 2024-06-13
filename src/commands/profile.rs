// src/commands/profile.rs
use crate::config::loader;
use crate::config::types::Profile;
use crate::constants;
use std::fs;


pub fn create_profile(profile_name: &str) {
    let mut config = loader::load(constants::CONFIG_FILE).unwrap_or_default();
    config.profiles.insert(profile_name.to_string(), Profile::default());
    loader::save(&config, constants::CONFIG_FILE).unwrap();
    println!("Profile {} created.", profile_name);
}

pub fn list_profiles() {
    let config = loader::load(constants::CONFIG_FILE).unwrap_or_default();
    for profile in config.profiles.keys() {
        println!("{}", profile);
    }
}

pub fn switch_profile(profile_name: &str) {
    let config = loader::load(constants::CONFIG_FILE).unwrap_or_default();
    if config.profiles.contains_key(profile_name) {
        fs::write("current_profile", profile_name).unwrap();
        println!("Switched to profile {}", profile_name);
    } else {
        println!("Profile {} does not exist", profile_name);
    }
}
