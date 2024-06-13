use log::info;
use std::fs::File;
use std::io::Write;

/// Initialize logging for the application.
pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create("dotsync.log")?;
    file.write_all(b"Log initialized\n")?;
    info!("Logging initialized.");
    Ok(())
}
