use crate::error::{BullastratorError, Result};
use crate::services::RedisConnection;
use anyhow::Context;
use bullastrator_storage::models::connection::InsecureConnectionCredentials;
use deadpool_redis::{
    Config as RedisConfig, ConnectionAddr, ConnectionInfo, ProtocolVersion, RedisConnectionInfo,
    Runtime,
};

pub fn create_redis_connection(
    connection: &InsecureConnectionCredentials,
) -> Result<RedisConnection> {
    if connection.host.is_empty() {
        return Err(BullastratorError::Validation(format!(
            "Redis host cannot be empty for {}",
            connection.id
        )));
    }
    let port = u16::try_from(connection.port)
        .with_context(|| format!("Invalid Redis port for {}", connection.id))?;
    let db = connection.db.unwrap_or(0);
    let db_u8 = u8::try_from(db)
        .with_context(|| format!("Invalid Redis database index for {}", connection.id))?;
    let tls = connection.is_tls_enabled.unwrap_or(false);
    let addr = if tls {
        ConnectionAddr::TcpTls {
            host: connection.host.clone(),
            port,
            insecure: false,
        }
    } else {
        ConnectionAddr::Tcp(connection.host.clone(), port)
    };
    let connection_info = ConnectionInfo {
        addr,
        redis: RedisConnectionInfo {
            db: i64::from(db),
            username: connection.username.clone(),
            password: connection.password.clone(),
            protocol: ProtocolVersion::RESP2,
        },
    };
    let pool = RedisConfig::from_connection_info(connection_info)
        .create_pool(Some(Runtime::Tokio1))
        .with_context(|| format!("Failed to create Redis pool for {}", connection.id))?;

    let bullmq_options = bullmq::options::RedisConnectionOptions {
        host: Some(connection.host.clone()),
        port: Some(port),
        username: connection.username.clone(),
        password: connection.password.clone(),
        db: Some(db_u8),
        tls,
        ..Default::default()
    };

    Ok(RedisConnection::with_options(
        pool,
        connection.id.clone(),
        connection
            .bullmq_prefix
            .clone()
            .unwrap_or_else(|| "bull".into()),
        bullmq_options,
    ))
}
