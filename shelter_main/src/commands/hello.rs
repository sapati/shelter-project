use crate::settings::Settings;
use clap::{ArgMatches, Command};

pub fn configure() -> Command {
    Command::new("hello").about("Hello World!")
}

pub fn handle(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("hello") {
        println!("Hello World!");
    }

    Ok(())
}
