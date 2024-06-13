// src/commands/setup.rs
use std::fs;
use std::path::Path;



pub fn setup_command(profile_name: &str) {
    // This function should create a file with "test content" for testing
    let file_path = Path::new(profile_name);
    fs::write(file_path, "test content").expect("Unable to write file");
}

pub fn setup_dotfile(src: &Path, dest: &Path) -> Result<(), std::io::Error> {
    if !dest.exists() {
        fs::copy(src, dest)?;
    }
    Ok(())
}
