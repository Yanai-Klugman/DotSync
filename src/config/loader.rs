use std::fs;
use crate::config::types::Config;
// use crate::constants::CONFIG_FILE;

pub fn load(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

/*
pub fn exists() -> bool {
    fs::metadata(CONFIG_FILE).is_ok()
}
*/

// Remove unused function for now
/*
pub fn save(config: &Config, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let content = toml::to_string(config)?;
    fs::write(path, content)?;
    Ok(())
}
*/

// Remove unused function for now
/*
pub fn default() -> Config {
    Config {
        global: GlobalConfig {
            dependencies: vec![],
        },
        profiles: HashMap::new(),
    }
}
*/
