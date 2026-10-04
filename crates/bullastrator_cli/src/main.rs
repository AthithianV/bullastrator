use anyhow::{Context, Result};
use bullastrator_core::state::AppState;
use bullastrator_web::server::serve;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::{
    io::{self, Write},
    net::IpAddr,
    path::{Path, PathBuf},
};

const DEFAULT_THEME: &str = "#00CADB";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    port: u16,
    database_path: PathBuf,
    theme_color: String,
    vpn_restricted: bool,
    bind_address: IpAddr,
}

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    let config_path = config_path()?;
    let config = if config_path.exists() {
        let saved = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Could not read {}", config_path.display()))?;
        let config: Config = serde_json::from_str(&saved)
            .with_context(|| format!("Invalid config at {}", config_path.display()))?;
        println!(
            "Using saved Bullastrator configuration: {}",
            config_path.display()
        );
        config
    } else {
        let config = onboarding()?;
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&config_path, serde_json::to_vec_pretty(&config)?)?;
        println!("Saved configuration to {}", config_path.display());
        config
    };

    if let Some(parent) = config.database_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let is_new_database = !config.database_path.exists();
    let options = SqliteConnectOptions::new()
        .filename(&config.database_path)
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new().connect_with(options).await?;
    if is_new_database {
        bullastrator_storage::initialize_database(&pool).await?;
    }

    let state = AppState::new(config.database_path.to_string_lossy().into_owned()).await?;

    let address = std::net::SocketAddr::new(config.bind_address, config.port);
    println!("Bullastrator is running at http://{address}");
    serve(state, address).await
}

fn init_tracing() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            tracing_subscriber::EnvFilter::new("bullastrator_core=debug,bullastrator_cli=debug")
        });
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_thread_ids(true)
        .init();
}

fn onboarding() -> Result<Config> {
    println!("Welcome to Bullastrator setup. Press Enter to accept a default.\n");
    let port = ask("App port", "3000")?
        .parse::<u16>()
        .context("Port must be between 0 and 65535")?;
    let default_db = default_database_path();
    let database_path = PathBuf::from(ask(
        "SQLite database path",
        &default_db.display().to_string(),
    )?);
    let theme_color = ask("Theme color", DEFAULT_THEME)?;
    let vpn_restricted = ask_yes_no("Restrict access to VPN/private network", false)?;
    let bind_address = ask("Bind address", "127.0.0.1")?
        .parse::<IpAddr>()
        .context("Invalid bind address")?;
    Ok(Config {
        port,
        database_path,
        theme_color,
        vpn_restricted,
        bind_address,
    })
}

fn ask(label: &str, default: &str) -> Result<String> {
    print!("{label} [{default}]: ");
    io::stdout().flush()?;
    let mut value = String::new();
    io::stdin().read_line(&mut value)?;
    let value = value.trim();
    Ok(if value.is_empty() {
        default.into()
    } else {
        value.into()
    })
}

fn ask_yes_no(label: &str, default: bool) -> Result<bool> {
    let hint = if default { "Y/n" } else { "y/N" };
    loop {
        print!("{label} [{hint}]: ");
        io::stdout().flush()?;
        let mut value = String::new();
        io::stdin().read_line(&mut value)?;
        match value.trim().to_ascii_lowercase().as_str() {
            "" => return Ok(default),
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => println!("Please answer y or n."),
        }
    }
}

fn config_path() -> Result<PathBuf> {
    Ok(dirs_next::config_dir()
        .context("Could not find a platform config directory")?
        .join("bullastrator/config.json"))
}

fn default_database_path() -> PathBuf {
    dirs_next::data_dir()
        .unwrap_or_else(|| Path::new(".").to_path_buf())
        .join("bullastrator/bullastrator.db")
}
