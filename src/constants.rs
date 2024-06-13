pub const CONFIG_DIR: &str = ".dotsync";
pub const CONFIG_FILE: &str = ".dotsync/dotfiles.toml";
pub const APP_NAME: &str = "dotsync";
pub const APP_VERSION: &str = "0.1.0";
pub const APP_AUTHOR: &str = "Yanai Klugman <yanaiklugman@gmail.com>";
pub const APP_ABOUT: &str = "A CLI tool to manage dotfiles declaratively across various environments.";

pub const SUBCOMMAND_SETUP: &str = "setup";
pub const DESC_SETUP: &str = "Setup the initial configuration";

pub const SUBCOMMAND_SYNC: &str = "sync";
pub const DESC_SYNC: &str = "Sync dotfiles for the specified profile";

pub const ARG_PROFILE: &str = "profile";
pub const ARG_DRY_RUN: &str = "dry-run";

pub const SUBCOMMAND_CREATE_PROFILE: &str = "create-profile";
pub const DESC_CREATE_PROFILE: &str = "Create a new dotfiles profile";

pub const SUBCOMMAND_LIST_PROFILES: &str = "list-profiles";
pub const DESC_LIST_PROFILES: &str = "List all available dotfiles profiles";

pub const SUBCOMMAND_SWITCH_PROFILE: &str = "switch-profile";
pub const DESC_SWITCH_PROFILE: &str = "Switch to a different dotfiles profile";
