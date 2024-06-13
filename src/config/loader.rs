use std::collections::HashMap;
use std::fs;
use crate::config::types::{Profile, Config, GlobalConfig};
// use crate::constants::CONFIG_FILE;

pub fn load(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    if !exists(path) {
        let default_config = default();
        save(&default_config, path)?;
    }
    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

pub fn save(config: &Config, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let contents = toml::to_string(config)?;
    fs::write(path, contents)?;
    Ok(())
}

pub fn exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

pub fn default() -> Config {
    let mut profiles = HashMap::new();
    profiles.insert(
        "personal".to_string(),
        Profile {
            dotfiles: {
                let mut map = HashMap::new();
                map.insert("/path/to/source/.bashrc".to_string(), "/path/to/destination/.bashrc".to_string());
                map.insert("/path/to/source/.vimrc".to_string(), "/path/to/destination/.vimrc".to_string());
                map
            },
        },
    );

    Config {
        profiles: HashMap::new(),
        global: GlobalConfig {
            default_profile: String::from("personal"),
        },
    }
}
