use clap::{Command, Arg, ArgAction};
use crate::commands::{sync::sync_command, profile::{create_profile, list_profiles, switch_profile}};
use crate::constants::{APP_NAME, APP_VERSION, APP_AUTHOR, APP_ABOUT, SUBCOMMAND_SYNC, DESC_SYNC, ARG_PROFILE, ARG_DRY_RUN, SUBCOMMAND_CREATE_PROFILE, DESC_CREATE_PROFILE, SUBCOMMAND_LIST_PROFILES, DESC_LIST_PROFILES, SUBCOMMAND_SWITCH_PROFILE, DESC_SWITCH_PROFILE};

mod commands;
mod config;
mod constants;
mod utils;

fn main() {
    let matches = Command::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHOR)
        .about(APP_ABOUT)
        .subcommand(
            Command::new(SUBCOMMAND_SYNC)
                .about(DESC_SYNC)
                .arg(Arg::new(ARG_PROFILE).required(true).default_value("personal"))
                .arg(Arg::new(ARG_DRY_RUN).long(ARG_DRY_RUN).action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new(SUBCOMMAND_CREATE_PROFILE)
                .about(DESC_CREATE_PROFILE)
                .arg(Arg::new(ARG_PROFILE).required(true)),
        )
        .subcommand(
            Command::new(SUBCOMMAND_LIST_PROFILES)
                .about(DESC_LIST_PROFILES),
        )
        .subcommand(
            Command::new(SUBCOMMAND_SWITCH_PROFILE)
                .about(DESC_SWITCH_PROFILE)
                .arg(Arg::new(ARG_PROFILE).required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some((SUBCOMMAND_SYNC, sync_matches)) => {
            let profile = sync_matches.get_one::<String>(ARG_PROFILE).unwrap();
            let dry_run = sync_matches.get_flag(ARG_DRY_RUN);
            sync_command(profile, dry_run);
        }
        Some((SUBCOMMAND_CREATE_PROFILE, profile_matches)) => {
            let profile = profile_matches.get_one::<String>(ARG_PROFILE).unwrap();
            create_profile(profile);
        }
        Some((SUBCOMMAND_LIST_PROFILES, _)) => {
            list_profiles();
        }
        Some((SUBCOMMAND_SWITCH_PROFILE, profile_matches)) => {
            let profile = profile_matches.get_one::<String>(ARG_PROFILE).unwrap();
            switch_profile(profile);
        }
        _ => eprintln!("Unknown command"),
    }
}
