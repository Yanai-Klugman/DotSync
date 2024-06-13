// src/commands/sync.rs
use crate::config::loader;
use crate::constants;
use std::fs;
use std::path::Path;
use std::error::Error;

pub fn sync_command(profile_name: &str, dry_run: bool) {
    let config = loader::load(constants::CONFIG_FILE).unwrap_or_default();
    let profile = config.profiles.get(profile_name).unwrap();
    for dotfile in &profile.dotfiles {
        if dry_run {
            println!("Would sync: {} -> {}", dotfile.source, dotfile.destination);
        } else {
            let _ = sync_dotfile(&dotfile.source, &dotfile.destination);
        }
    }
}

pub fn sync_dotfile(src: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    if Path::new(src).exists() {
        fs::copy(src, dest)?;
        Ok(())
    } else {
        Err(format!("Source file {} does not exist", src).into())
    }
}
