// use std::fs;
// use std::path::Path;
// use std::io::Result;

// Copy a file from source to destination.
/*
pub fn copy_file(src: &Path, dest: &Path) -> Result<()> {
    fs::copy(src, dest)?;
    Ok(())
}

/// Backup a file by renaming it with a .bak extension.
pub fn backup_file(file: &Path) -> Result<()> {
    let backup_path = file.with_extension("bak");
    fs::rename(file, backup_path)?;
    Ok(())
}
*/