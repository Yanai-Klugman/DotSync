mod commands;
mod config;
mod constants;
mod utils;

use clap::{Arg, Command};
use commands::sync::sync_command;
use constants::*;

fn main() {
    let matches = Command::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHOR)
        .about(APP_ABOUT)
        .subcommand(
            Command::new(SUBCOMMAND_SYNC)
                .about(DESC_SYNC)
                .arg(Arg::new(ARG_PROFILE).required(true).default_value("personal"))
                .arg(Arg::new(ARG_DRY_RUN).long(ARG_DRY_RUN).action(clap::ArgAction::SetTrue)),
        )
        .get_matches();

    match matches.subcommand() {
        Some((SUBCOMMAND_SYNC, sync_matches)) => {
            let profile = sync_matches.get_one::<String>(ARG_PROFILE).unwrap();
            let dry_run = sync_matches.get_flag(ARG_DRY_RUN);
            sync_command(profile, dry_run);
        }
        _ => eprintln!("No valid subcommand was used"),
    }
}
