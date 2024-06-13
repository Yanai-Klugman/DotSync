use std::fs;
use std::path::Path;
use crate::config::loader;
use crate::constants::CONFIG_FILE;

pub fn sync_command(profile_name: &str, dry_run: bool) {
    let config = loader::load(CONFIG_FILE).unwrap();
    let profile = config.profiles.get(profile_name).expect("Profile not found");

    for (source, destination) in &profile.dotfiles {
        if dry_run {
            println!("Would sync: {} -> {}", source, destination);
        } else {
            sync_dotfile(source, destination);
        }
    }
}

fn sync_dotfile(source: &str, destination: &str) {
    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    if source_path.exists() {
        if let Err(e) = fs::copy(source_path, destination_path) {
            eprintln!("Failed to sync {} to {}: {}", source, destination, e);
        } else {
            println!("Synced {} to {}", source, destination);
        }
    } else {
        eprintln!("Source file {} does not exist", source);
    }
}
