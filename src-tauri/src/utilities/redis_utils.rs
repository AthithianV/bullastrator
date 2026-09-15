use anyhow::{Context, Result, anyhow};
use url::form_urlencoded;
use uuid::Uuid;

use crate::{
    AppState,
    model::connection_model::ReadConnectionModel,
    repository::connection_repository::ConnectionRepository,
    utilities::{app_state::ConnectionPool, encryption::decrypt_password},
};

use deadpool_redis::{Config, Pool, PoolConfig, Runtime, Timeouts};

async fn init_redis(redis_url: String) -> Result<Pool> {
    // 1. Configure the pool
    let mut cfg = Config::from_url(redis_url);

    cfg.pool = Some(PoolConfig {
        max_size: 10,
        timeouts: Timeouts {
            // Time to wait for a slot/connection to become available in the pool
            wait: Some(std::time::Duration::from_secs(5)),
            // Time to wait for the actual TLS/TCP handshake to complete
            create: Some(std::time::Duration::from_secs(3)),
            // Time to wait for a "health check" on an existing connection
            recycle: Some(std::time::Duration::from_secs(2)),
            ..Default::default()
        },
        ..Default::default()
    });

    // 2. Build the pool (using Tokio runtime)
    let pool = cfg
        .create_pool(Some(Runtime::Tokio1))
        .context("Failed to create pool")?;

    Ok(pool)
}

pub async fn get_or_create_redis_connection(
    state: &AppState,
    connection_id: Uuid,
) -> Result<ConnectionPool> {
    if let Ok(conn) = state.get_redis_connection(&connection_id).await {
        return Ok(conn);
    }

    let db_conn = state
        .get_app_db_connection()
        .await
        .map_err(anyhow::Error::msg)
        .context("DB not initialized")?;

    let connection_repo = ConnectionRepository::new(&db_conn);
    let connection = connection_repo
        .get_by_id_with_password(connection_id)
        .await
        .context("Connection not found")?
        .ok_or_else(|| anyhow!("Connection ID {} not found", connection_id))?;

    let decrypted_password = match connection.password {
        Some(ref encrypted_hex) => {
            let decrypted = decrypt_password(encrypted_hex)
                .map_err(|e| anyhow::anyhow!("Failed to decrypt password: {}", e))?;
            Some(decrypted)
        }
        None => None,
    };

    let redis_url = create_redis_url(
        connection.host.clone(),
        connection.port.clone(),
        connection.username.clone(),
        decrypted_password,
        connection.db,
        connection.is_tls_enabled,
    )
    .context("Failed to generate Redis URL")?;

    let pool = init_redis(redis_url).await?;

    let new_pool = ConnectionPool {
        pool: pool.clone(),
        metadata: ReadConnectionModel {
            id: connection.id,
            workspace_id: connection.workspace_id,
            name: connection.name,
            host: connection.host,
            port: connection.port,
            username: connection.username,
            db: connection.db,
            is_tls_enabled: connection.is_tls_enabled,
            created_at: connection.created_at,
            last_synced_at: connection.last_synced_at,
            bullmq_prefix: connection.bullmq_prefix,
            color: connection.color,
            label: connection.label,
        },
    };

    // 4. UPDATE STATE: Store for future use
    state
        .add_redis_connection(connection_id.clone(), new_pool)
        .await;

    Ok(state.get_redis_connection(&connection_id).await?)
}

pub fn create_redis_url(
    host: String,
    port: i32,
    username: Option<String>,
    password: Option<String>,
    db_index: i32,
    use_tls: bool,
) -> Result<String> {
    let scheme = if use_tls { "rediss" } else { "redis" };

    let u = username.unwrap_or_default();
    let p = password.unwrap_or_default();

    let credentials_part = if u.is_empty() && p.is_empty() {
        String::new()
    } else {
        let encoded_u = form_urlencoded::byte_serialize(u.as_bytes()).collect::<String>();
        let encoded_p = form_urlencoded::byte_serialize(p.as_bytes()).collect::<String>();

        if u.is_empty() {
            format!(":{}@", encoded_p)
        } else {
            format!("{}:{}@", encoded_u, encoded_p)
        }
    };

    Ok(format!(
        "{}://{}{}:{}/{}#insecure",
        scheme, credentials_part, host, port, db_index
    ))
}

pub fn parse_u32(val: &Option<String>) -> u32 {
    val.as_ref()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0)
}

pub fn parse_u64(val: &Option<String>) -> u64 {
    val.as_ref()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0)
}
