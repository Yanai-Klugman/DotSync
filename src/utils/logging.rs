pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    Ok(())
}
