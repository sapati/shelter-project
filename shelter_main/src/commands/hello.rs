use crate::settings::Settings;
use clap::{ArgMatches, Command};

pub const COMMAND_NAME: &str = "hello";

pub fn configure() -> Command {
    Command::new(COMMAND_NAME).about("Hello World!")
}

pub fn handle(_matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    println!("Hello World!");

    Ok(())
}
