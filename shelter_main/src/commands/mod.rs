mod hello;
mod serve;

use clap::{ArgMatches, Command};

pub fn configure(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    hello::handle(matches)?;
    serve::handle(matches)?;

    Ok(())
}
