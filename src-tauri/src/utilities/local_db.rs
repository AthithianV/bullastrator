use dirs_next::data_dir;
use sea_orm::ConnectOptions;
use sea_orm::{Database, DatabaseConnection};
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use migration::{Migrator, MigratorTrait};

use anyhow::{Context, Result};

pub async fn initialize_local_db() -> Result<DatabaseConnection> {
    const DB_DIRECTORY_NAME: &str = "asyncian";
    const DB_FILE_NAME: &str = "data.db";

    let app_data_path: PathBuf = data_dir().context("Cannot find app data dir")?;

    let db_dir = Path::new(&app_data_path).join(DB_DIRECTORY_NAME);
    if !db_dir.exists() {
        std::fs::create_dir_all(&db_dir)
            .with_context(|| format!("Failed to create database directory: {:?}", db_dir))?;
    }

    let db_path = Path::new(&db_dir).join(DB_FILE_NAME);
    let db_exists = db_path.exists();
    println!("Database exists: {}", db_exists);

    let db_url = format!(
        "sqlite://{}/{}?mode=rwc",
        db_dir.to_str().unwrap(),
        DB_FILE_NAME
    );
    println!("Connecting to database with URL: {}", db_url);

    let mut opt = ConnectOptions::new(db_url.to_owned());

    opt.max_connections(10)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8));

    opt.sqlx_logging(false);
    // opt.sqlx_logging(true)
    //    .sqlx_logging_level(LevelFilter::Debug)
    //    .max_slow_statements_logging(Duration::from_secs(1));

    let db: DatabaseConnection = Database::connect(opt)
        .await
        .context("Failed to connect to database: {}")?;

    // Only run migrations if database doesn't exist or has pending migrations
    if !db_exists {
        Migrator::up(&db, None)
            .await
            .context("Migration failed while creating new DB")?;
    } else {
        let pending = Migrator::get_pending_migrations(&db)
            .await
            .context("Failed to check pending migrations")?;

        if !pending.is_empty() {
            println!(
                "Found {} pending migrations, running them...",
                pending.len()
            );
            Migrator::up(&db, None)
                .await
                .context("Migration failed while applying pending migrations")?;
        } else {
            println!("Database is up to date, no migrations needed");
        }
    }

    println!("Database initialization completed");
    Ok(db)
}
