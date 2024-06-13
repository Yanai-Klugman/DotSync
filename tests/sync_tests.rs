// tests/sync_tests.rs
use tempfile::NamedTempFile;
use std::io::Write;
use std::fs;

#[test]
fn test_sync_dotfile() {
    // Create a temporary source file
    let mut src_file = NamedTempFile::new().unwrap();
    writeln!(src_file, "Hello, world!").unwrap();
    let src_path = src_file.path().to_str().unwrap().to_string();

    // Create a temporary destination path
    let dest_file = NamedTempFile::new().unwrap();
    let dest_path = dest_file.path().to_str().unwrap().to_string();

    // Call the function to be tested
    dotsync::commands::sync::sync_dotfile(&src_path, &dest_path).unwrap();

    // Verify the contents of the destination file
    let dest_contents = fs::read_to_string(dest_path).unwrap();
    assert_eq!(dest_contents, "Hello, world!\n");
}
