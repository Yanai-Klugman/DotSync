use crate::config::loader;
use crate::constants;
use std::fs;
use std::path::Path;



pub fn sync_command(profile_name: &str, dry_run: bool) {
    let config = loader::load(constants::CONFIG_FILE).unwrap_or_default();
    let profile = config.profiles.get(profile_name).unwrap();
    for dotfile in &profile.dotfiles {
        if dry_run {
            println!("Would sync: {} -> {}", dotfile.source, dotfile.destination);
        } else {
            sync_dotfile(Path::new(&dotfile.source), Path::new(&dotfile.destination)).unwrap();
        }
    }
}

pub fn sync_dotfile(src: &Path, dest: &Path) -> Result<(), std::io::Error> {
    fs::copy(src, dest)?;
    Ok(())
}
