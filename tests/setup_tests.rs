use std::fs;
use mktemp::Temp;
use dotsync::commands::setup::setup_command;

#[test]
fn test_setup_dotfile() {
    let dir = Temp::new_dir().unwrap();
    let file_path = dir.to_path_buf().join("test_profile");

    // Call the setup_command function to create the file
    setup_command(file_path.to_str().unwrap());

    // Read the file content
    let content = fs::read_to_string(&file_path).expect("Unable to read file");

    // Assert the content is "test content"
    assert_eq!(content, "test content");
}
