use clap::{Arg, Command};
use dotenv::dotenv;
use shelter_main::commands;
use shelter_main::settings;

pub fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let mut command = Command::new("Dog Shelter sample application")
        .version("1.0")
        .author("Sandor Apati <sapati@gmail.com>")
        .about("A sample application to experiment with Rust-based microservices")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Configuration file location")
                .default_value("config.json"),
        );

    command = commands::configure(command);

    let matches = command.get_matches();

    let config_location = matches
        .get_one::<String>("config")
        .map(|s| s.as_str())
        .unwrap_or("");

    let settings = settings::Settings::new(config_location, "SHELTER")?;

    commands::handle(&matches, &settings)?;

    Ok(())
}
