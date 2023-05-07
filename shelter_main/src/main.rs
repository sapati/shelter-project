use clap::{Arg, Command};
use dotenv::dotenv;

pub fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let command = Command::new("Dog Shelter sample application")
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

    let _matches = command.get_matches();

    Ok(())
}
