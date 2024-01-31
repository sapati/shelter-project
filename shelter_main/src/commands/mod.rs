mod createadmin;
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
        .subcommand(createadmin::configure())
        .arg_required_else_help(true)
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some((cmd, matches)) = matches.subcommand() {
        match cmd {
            hello::COMMAND_NAME => hello::handle(matches, settings)?,
            serve::COMMAND_NAME => serve::handle(matches, settings)?,
            migrate::COMMAND_NAME => migrate::handle(matches, settings)?,
            createadmin::COMMAND_NAME => createadmin::handle(matches, settings)?,
            &_ => {}
        }
    }

    Ok(())
}
