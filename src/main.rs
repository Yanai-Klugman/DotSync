use clap::{Arg, Command};
use dotsync::{commands::{profile::{create_profile, list_profiles, switch_profile}, setup::setup_command, sync::sync_command}, constants};

fn main() {
    let matches = Command::new(constants::APP_NAME)
        .version(constants::APP_VERSION)
        .author(constants::APP_AUTHOR)
        .about(constants::APP_ABOUT)
        .subcommand(
            Command::new(constants::SUBCOMMAND_SETUP)
                .about(constants::DESC_SETUP)
                .arg(Arg::new(constants::ARG_PROFILE).required(true).default_value("personal")),
        )
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
        .subcommand(
            Command::new(constants::SUBCOMMAND_LIST_PROFILES)
                .about(constants::DESC_LIST_PROFILES),
        )
        .subcommand(
            Command::new(constants::SUBCOMMAND_SWITCH_PROFILE)
                .about(constants::DESC_SWITCH_PROFILE)
                .arg(Arg::new(constants::ARG_PROFILE).required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some((constants::SUBCOMMAND_SETUP, setup_matches)) => {
            let profile = setup_matches.get_one::<String>(constants::ARG_PROFILE).unwrap();
            setup_command(profile);
        }
        Some((constants::SUBCOMMAND_SYNC, sync_matches)) => {
            let profile = sync_matches.get_one::<String>(constants::ARG_PROFILE).unwrap();
            let dry_run = sync_matches.get_flag(constants::ARG_DRY_RUN);
            sync_command(profile, dry_run);
        }
        Some((constants::SUBCOMMAND_CREATE_PROFILE, create_profile_matches)) => {
            let profile = create_profile_matches.get_one::<String>(constants::ARG_PROFILE).unwrap();
            create_profile(profile);
        }
        Some((constants::SUBCOMMAND_LIST_PROFILES, _)) => {
            list_profiles();
        }
        Some((constants::SUBCOMMAND_SWITCH_PROFILE, switch_profile_matches)) => {
            let profile = switch_profile_matches.get_one::<String>(constants::ARG_PROFILE).unwrap();
            switch_profile(profile);
        }
        _ => {}
    }
}
