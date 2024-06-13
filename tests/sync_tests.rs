use std::fs;
use mktemp::Temp;
use dotsync::commands::sync::sync_dotfile;

#[test]
fn test_sync_dotfile() {
    let src_file = Temp::new_file().unwrap();
    let dest_file = Temp::new_file().unwrap();

    // Write something to the source file
    fs::write(&src_file, "test content").unwrap();

    // Sync the dotfile
    sync_dotfile(src_file.to_path_buf().as_path(), dest_file.to_path_buf().as_path()).unwrap();

    // Read the destination file content
    let content = fs::read_to_string(dest_file.to_path_buf().as_path()).unwrap();

    // Assert the content is "test content"
    assert_eq!(content, "test content");
}
