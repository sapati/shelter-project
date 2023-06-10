use crate::settings::Settings;
use clap::{ArgMatches, Command};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

pub fn configure() -> Command {
    Command::new("migrate").about("Run database migrations")
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(_matches) = matches.subcommand_matches("migrate") {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let db_url = settings.database.url.clone().unwrap_or("".to_string());
                let conn = Database::connect(db_url)
                    .await
                    .expect("Database connection failed");
                Migrator::up(&conn, None).await.unwrap();
            });
    }

    Ok(())
}
