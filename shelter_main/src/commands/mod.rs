mod hello;
mod migrate;
mod serve;

use crate::settings::Settings;
use clap::{ArgMatches, Command};

pub fn configure(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
        .subcommand(migrate::configure())
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    hello::handle(matches, settings)?;
    serve::handle(matches, settings)?;
    migrate::handle(matches, settings)?;

    Ok(())
}
