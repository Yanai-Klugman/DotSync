use crate::config::loader;
use crate::config::types::Profile;
use crate::constants::CONFIG_FILE;

pub fn create_profile(profile: &str) {
    let mut config = loader::load(CONFIG_FILE).unwrap();
    if config.profiles.contains_key(profile) {
        eprintln!("Profile {} already exists.", profile);
    } else {
        config.profiles.insert(profile.to_string(), Profile::default());
        loader::save(&config, CONFIG_FILE).unwrap();
        println!("Profile {} created.", profile);
    }
}

pub fn list_profiles() {
    let config = loader::load(CONFIG_FILE).unwrap();
    for profile in config.profiles.keys() {
        println!("{}", profile);
    }
}

pub fn switch_profile(profile: &str) {
    let config = loader::load(CONFIG_FILE).unwrap();
    if config.profiles.contains_key(profile) {
        // Logic to switch profiles
        println!("Switched to profile {}", profile);
    } else {
        eprintln!("Profile {} does not exist.", profile);
    }
}
