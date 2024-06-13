use crate::config::loader;
use crate::constants::CONFIG_FILE;

pub fn sync_command(profile: &str, dry_run: bool) {
    if dry_run {
        println!("Would sync profile: {}", profile);
    } else {
        println!("Syncing profile: {}", profile);
        let _config = loader::load(CONFIG_FILE).unwrap();
        // Perform sync using `_config`
    }
}
