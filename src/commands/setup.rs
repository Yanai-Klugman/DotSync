use crate::config::loader;
use crate::constants;
use log::info;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn setup_command(profile_name: &str, dry_run: bool) -> Result<(), Box<dyn Error>> {
    let config = loader::load(Path::new(constants::CONFIG_FILE))?;
    let profile = config.profiles.get(profile_name)
        .ok_or_else(|| format!("Profile '{}' not found in configuration", profile_name))?;
    for (source, destination) in &profile.dotfiles {
        if dry_run {
            info!("Would sync: {} -> {}", source, destination);
        } else {
            setup_dotfile(Path::new(source), Path::new(destination))?;
        }
    }
    Ok(())
}

pub fn setup_dotfile(src: &Path, dest: &Path) -> Result<(), std::io::Error> {
    fs::copy(src, dest)?;
    Ok(())
}
