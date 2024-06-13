use clap::{Arg, Command};
use tokio::main;

mod commands;
mod config;
mod utils;
mod constants;

#[main]
async fn main() {
    utils::logging::init_logging().unwrap();

    let matches = Command::new(constants::APP_NAME)
        .version(constants::APP_VERSION)
        .author(constants::APP_AUTHOR)
        .about(constants::APP_ABOUT)
        .subcommand(
            Command::new(constants::SUBCOMMAND_SYNC)
                .about(constants::DESC_SYNC)
                .arg(Arg::new(constants::ARG_PROFILE).required(true).default_value("personal"))
                .arg(Arg::new(constants::ARG_DRY_RUN).long(constants::ARG_DRY_RUN).action(clap::ArgAction::SetTrue)),
        )
        .get_matches();

    match matches.subcommand() {
        Some((constants::SUBCOMMAND_SYNC, sync_matches)) => {
            let profile = sync_matches.get_one::<String>(constants::ARG_PROFILE).unwrap();
            let dry_run = sync_matches.get_flag(constants::ARG_DRY_RUN);
            commands::sync::sync(profile, dry_run).await.unwrap();
        }
        _ => {}
    }
}
