use anyhow::{Context, Result, bail};
use semver::Version;
use std::time::Duration;
use tokio::time::timeout;

use crate::{
    model::connection_model::CreateConnectionModel,
    utilities::{app_state::ConnectionPool, redis_utils::create_redis_url},
};

pub async fn start_health_check_service(connection_pool: &ConnectionPool) -> Result<bool> {
    // 1. Get a connection from the pool
    // We wrap this in a timeout too, in case the pool is full/saturated
    let redis_conn_result = timeout(Duration::from_secs(1), connection_pool.pool.get()).await;

    let mut redis_conn = match redis_conn_result {
        Ok(Ok(conn)) => conn,
        _ => return Ok(false), // Failed to get connection or timed out
    };

    // 2. Wrap the PING in a strict 1-second timeout
    let ping_result = timeout(
        Duration::from_secs(1),
        redis::cmd("PING").query_async::<String>(&mut *redis_conn),
    )
    .await;

    // The result is true ONLY if the ping finished within 1s AND returned Ok
    let is_alive = match ping_result {
        Ok(Ok(_)) => true,
        _ => false, // Either timed out or returned a Redis error
    };

    Ok(is_alive)
}

pub async fn test_redis_connection_service(redis_url: String) -> Result<bool, String> {
    let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;

    // Use multiplexed for better async performance
    let conn_result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        client.get_multiplexed_async_connection(),
    )
    .await;

    match conn_result {
        Ok(Ok(mut conn)) => {
            let response: String = redis::cmd("PING")
                .query_async(&mut conn)
                .await
                .map_err(|e| e.to_string())?;

            Ok(response == "PONG")
        }
        Ok(Err(e)) => Err(format!("Connection error: {}", e)),
        Err(_) => Err("Timeout".to_string()),
    }
}

pub async fn check_redis_version(data: &CreateConnectionModel) -> Result<()> {
    let redis_url = create_redis_url(
        data.host.clone(),
        data.port,
        data.username.clone(),
        data.password.clone(),
        data.db.unwrap_or(0),
        data.is_tls_enabled,
    )?;

    let client = redis::Client::open(redis_url).context("Failed to create Redis client")?;

    // Set a 5-second timeout for the connection and the INFO command
    let info: String = timeout(Duration::from_secs(5), async {
        let mut con = client.get_multiplexed_async_connection().await?;
        let res: String = redis::cmd("INFO")
            .arg("server")
            .query_async(&mut con)
            .await?;
        Ok::<String, redis::RedisError>(res)
    })
    .await
    .map_err(|_| anyhow::anyhow!("Connection timed out after 5 seconds"))?
    .context("Failed to execute INFO command on Redis")?;

    // Extract version string
    let version_line = info
        .lines()
        .find(|line| line.starts_with("redis_version:"))
        .ok_or_else(|| anyhow::anyhow!("Could not find redis_version in INFO output"))?;

    let version = version_line
        .split(':')
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Malformed redis_version string"))?
        .trim();

    let current_version = Version::parse(version)?;
    let required_version = Version::parse("6.2.0")?;

    if current_version < required_version {
        bail!(
            "Redis version {} is unsupported. Please use 6.2 or higher.",
            version
        );
    }

    Ok(())
}
