use crate::settings::Settings;
use clap::{ArgMatches, Command};
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

pub const COMMAND_NAME: &str = "migrate";

pub fn configure() -> Command {
    Command::new(COMMAND_NAME).about("Run database migrations")
}

pub fn handle(_matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
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

    Ok(())
}
