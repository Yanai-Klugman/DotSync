// src/main.rs
use clap::{Arg, Command};
use dotsync::commands::{profile, sync};
use dotsync::constants;

fn main() {
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
        .subcommand(
            Command::new(constants::SUBCOMMAND_CREATE_PROFILE)
                .about(constants::DESC_CREATE_PROFILE)
                .arg(Arg::new(constants::ARG_PROFILE).required(true)),
        )
        .subcommand(Command::new(constants::SUBCOMMAND_LIST_PROFILES).about(constants::DESC_LIST_PROFILES))
        .subcommand(
            Command::new(constants::SUBCOMMAND_SWITCH_PROFILE)
                .about(constants::DESC_SWITCH_PROFILE)
                .arg(Arg::new(constants::ARG_PROFILE).required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some((constants::SUBCOMMAND_SYNC, sync_matches)) => {
            let profile = sync_matches.get_one::<String>(constants::ARG_PROFILE).unwrap();
            let dry_run = sync_matches.get_flag(constants::ARG_DRY_RUN);
            sync::sync_command(profile, dry_run);
        }
        Some((constants::SUBCOMMAND_CREATE_PROFILE, create_matches)) => {
            let profile = create_matches.get_one::<String>(constants::ARG_PROFILE).unwrap();
            profile::create_profile(profile);
        }
        Some((constants::SUBCOMMAND_LIST_PROFILES, _)) => {
            profile::list_profiles();
        }
        Some((constants::SUBCOMMAND_SWITCH_PROFILE, switch_matches)) => {
            let profile = switch_matches.get_one::<String>(constants::ARG_PROFILE).unwrap();
            profile::switch_profile(profile);
        }
        _ => {}
    }
}
