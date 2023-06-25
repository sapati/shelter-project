use crate::settings::Settings;
use anyhow::anyhow;
use argon2::Argon2;
use clap::{Arg, ArgMatches, Command};
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHasher, SaltString};
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, Database, EntityTrait};
use serde_json::json;

pub fn configure() -> Command {
    Command::new("createadmin")
        .about("Create the default admin user")
        .arg(
            Arg::new("password")
                .short('p')
                .long("password")
                .value_name("PASSWORD")
                .help("Password for admin user")
                .default_value("Pa$$wd123"),
        )
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches("createadmin") {
        let password = matches.get_one::<String>("password").unwrap();

        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let db_url = settings.database.url.clone().unwrap_or("".to_string());
                let conn = Database::connect(db_url)
                    .await
                    .expect("Database connection failed");

                let admins: Vec<entity::user::Model> = entity::user::Entity::find()
                    .filter(entity::user::Column::Username.eq("admin"))
                    .all(&conn)
                    .await?;

                if !admins.is_empty() {
                    println!("Admin user already exists");
                    return Ok(());
                }

                let encrypted_password = encrypt_password(password)?;

                let admin_model = entity::user::ActiveModel::from_json(json!({
                    "username": "admin",
                    "password": encrypted_password,
                }))?;

                if let Ok(_admin) = admin_model.save(&conn).await {
                    println!("Admin user created");
                } else {
                    println!("Failed to create admin user");
                }

                Ok::<(), anyhow::Error>(())
            })?;
    }

    Ok(())
}

fn encrypt_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    if let Ok(hash) = argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash.to_string())
    } else {
        Err(anyhow!("Failed to hash password"))
    }
}
