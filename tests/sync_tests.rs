use std::fs;
use mktemp::Temp;

#[cfg(test)]
mod sync_tests {
    use super::*;

    #[test]
    fn test_sync_dotfile() {
        // Create temporary source and destination files
        let src_file = Temp::new_file().unwrap();
        let dest_file = Temp::new_file().unwrap();

        // Write some content to the source file
        let src_path = src_file.to_path_buf();
        fs::write(&src_path, "test content").unwrap();

        // The destination file path
        let dest_path = dest_file.to_path_buf();

        // Call the sync_dotfile function
        dotsync::commands::sync::sync_dotfile(&src_path, &dest_path).unwrap();

        // Verify the content has been copied
        let dest_content = fs::read_to_string(&dest_path).unwrap();
        assert_eq!(dest_content, "test content");
    }
}
