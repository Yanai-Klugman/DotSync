use std::fs;
use std::path::Path;
use std::io::Result;

// Copy a file from source to destination.
pub fn copy_file(src: &Path, dest: &Path) -> Result<()> {
    fs::copy(src, dest)?;
    Ok(())
}
