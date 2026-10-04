use anyhow::{bail, Context, Result};
use semver::Version;
use std::time::Duration;
use tokio::time::timeout;

use bullastrator_storage::models::CreateConnection;

pub async fn start_health_check_service(pool: &deadpool_redis::Pool) -> Result<bool> {
    let mut connection = match timeout(Duration::from_secs(1), pool.get()).await {
        Ok(Ok(connection)) => connection,
        _ => return Ok(false),
    };
    Ok(matches!(
        timeout(
            Duration::from_secs(1),
            redis::cmd("PING").query_async::<String>(&mut *connection)
        )
        .await,
        Ok(Ok(_))
    ))
}

pub async fn test_redis_connection_service(redis_url: String) -> Result<bool, String> {
    let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
    let mut connection = tokio::time::timeout(
        Duration::from_secs(5),
        client.get_multiplexed_async_connection(),
    )
    .await
    .map_err(|_| "Timeout".to_string())
    .and_then(|r| r.map_err(|e| e.to_string()))?;
    let response: String = redis::cmd("PING")
        .query_async(&mut connection)
        .await
        .map_err(|e| e.to_string())?;
    Ok(response == "PONG")
}

pub async fn check_redis_version(data: &CreateConnection) -> Result<()> {
    let url = create_redis_url(
        &data.host,
        data.port,
        data.username.as_deref(),
        data.password.as_deref(),
        data.db.unwrap_or(0),
        data.is_tls_enabled,
    )?;
    let client = redis::Client::open(url).context("Failed to create Redis client")?;
    let info: String = timeout(Duration::from_secs(5), async {
        let mut connection = client.get_multiplexed_async_connection().await?;
        redis::cmd("INFO")
            .arg("server")
            .query_async(&mut connection)
            .await
    })
    .await
    .map_err(|_| anyhow::anyhow!("Connection timed out after 5 seconds"))??;
    let version = info
        .lines()
        .find_map(|line| line.strip_prefix("redis_version:"))
        .context("Could not find redis_version in INFO output")?
        .trim();
    if Version::parse(version)? < Version::parse("6.2.0")? {
        bail!(
            "Redis version {} is unsupported. Please use Redis 6.2 or higher.",
            version
        );
    }
    Ok(())
}

pub fn create_redis_url(
    host: &str,
    port: i32,
    username: Option<&str>,
    password: Option<&str>,
    db: i32,
    tls: bool,
) -> Result<String> {
    let scheme = if tls { "rediss" } else { "redis" };
    let mut url = url::Url::parse(&format!("{scheme}://{host}:{port}/{db}"))?;
    if let Some(username) = username {
        url.set_username(username)
            .map_err(|_| anyhow::anyhow!("Invalid Redis username"))?;
    }
    if let Some(password) = password {
        url.set_password(Some(password))
            .map_err(|_| anyhow::anyhow!("Invalid Redis password"))?;
    }
    Ok(url.to_string())
}
