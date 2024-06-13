use std::fs;
use std::path::Path;
use crate::config::types::{Config, GlobalConfig, Profiles, Profile, Dotfile};

pub const CONFIG_FILE: &str = "dotfiles.toml";

/// Load configuration from a TOML file.
pub fn load(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

/// Check if the configuration file exists.
pub fn exists() -> bool {
    Path::new(CONFIG_FILE).exists()
}

/// Save the configuration to a TOML file.
pub fn save(config: &Config, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = toml::to_string(config)?;
    fs::write(path, content)?;
    Ok(())
}

/// Create a default configuration.
pub fn default() -> Config {
    Config {
        global: GlobalConfig {
            dependencies: vec!["git".to_string(), "curl".to_string()],
        },
        profiles: Profiles {
            personal: Profile {
                dotfiles: vec![
                    Dotfile {
                        source: "~/.dotfiles/default/bashrc".to_string(),
                        destination: "~/.bashrc".to_string(),
                    },
                    Dotfile {
                        source: "~/.dotfiles/default/nvim/init.lua".to_string(),
                        destination: "~/.config/nvim/init.lua".to_string(),
                    },
                ],
                dependencies: vec!["bash".to_string(), "neovim".to_string()],
            },
        },
    }
}
