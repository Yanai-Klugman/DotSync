use crate::config::types::Config;
use crate::constants;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::Path;

pub fn load(file_path: &Path) -> Result<Config, Box<dyn Error>> {
    let mut file = fs::File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

pub fn save(config: &Config, path: &Path) -> Result<(), Box<dyn Error>> {
    let content = serde_json::to_string_pretty(config)?;
    fs::write(path, content)?;
    Ok(())
}

pub fn exists() -> bool {
    fs::metadata(Path::new(constants::CONFIG_FILE)).is_ok()
}
