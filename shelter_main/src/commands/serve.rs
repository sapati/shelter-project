use clap::{value_parser, Arg, ArgMatches, Command};

pub fn configure() -> Command {
    Command::new("serve").about("Start HTTP server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("TCP port to listen on")
            .default_value("8080")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches("serve") {
        let port: u16 = *matches.get_one("port").unwrap_or(&8080);

        println!("TBD: start the webserver on port {}", port)
    }

    Ok(())
}
