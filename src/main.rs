use dotsync::commands::profile::{create_profile, list_profiles, switch_profile};
use dotsync::commands::sync::sync_command;
use clap::{Arg, Command};
use std::error::Error;

const COMMAND_SYNC: &str = "sync";
const COMMAND_CREATE_PROFILE: &str = "create-profile";
const COMMAND_LIST_PROFILES: &str = "list-profiles";
const COMMAND_SWITCH_PROFILE: &str = "switch-profile";

const ARG_PROFILE: &str = "profile";

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let matches = Command::new("dotsync")
        .subcommand(Command::new(COMMAND_SYNC).arg(Arg::new(ARG_PROFILE).required(true)))
        .subcommand(Command::new(COMMAND_CREATE_PROFILE).arg(Arg::new(ARG_PROFILE).required(true)))
        .subcommand(Command::new(COMMAND_LIST_PROFILES))
        .subcommand(Command::new(COMMAND_SWITCH_PROFILE).arg(Arg::new(ARG_PROFILE).required(true)))
        .get_matches();

    if let Some((subcommand, sub_m)) = matches.subcommand() {
        match subcommand {
            COMMAND_SYNC => {
                if let Some(profile_name) = sub_m.get_one::<String>(ARG_PROFILE) {
                    sync_command(profile_name, false)?;
                }
            }
            COMMAND_CREATE_PROFILE => {
                if let Some(profile_name) = sub_m.get_one::<String>(ARG_PROFILE) {
                    create_profile(profile_name)?;
                }
            }
            COMMAND_LIST_PROFILES => {
                list_profiles()?;
            }
            COMMAND_SWITCH_PROFILE => {
                if let Some(profile_name) = sub_m.get_one::<String>(ARG_PROFILE) {
                    switch_profile(profile_name)?;
                }
            }
            _ => eprintln!("Unknown command"),
        }
    }

    Ok(())
}
