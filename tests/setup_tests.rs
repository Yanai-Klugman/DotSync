#[cfg(test)]
mod tests {
    use dotsync::commands::sync::sync_dotfile;
    use std::path::PathBuf;

    #[test]
    fn test_setup_dotfile() {
        let source = PathBuf::from("/home/yanai/Projects/dotsync/tests/source/.bashrc");
        let dest = PathBuf::from("/home/yanai/Projects/dotsync/tests/destination/.bashrc");
        assert!(sync_dotfile(&source, &dest).is_ok());
    }
}
