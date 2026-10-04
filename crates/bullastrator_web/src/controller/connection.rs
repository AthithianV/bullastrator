use crate::controller::ApiResult;
use axum::{
    Json,
    extract::{Path, State},
};
use bullastrator_core::{services::connection as connection_service, state::AppState};
use bullastrator_storage::models::CreateConnection;
use serde::Deserialize;

pub(crate) async fn test_redis(Json(request): Json<RedisUrlRequest>) -> ApiResult<Json<bool>> {
    Ok(Json(
        connection_service::test_redis_connection_service(request.redis_url)
            .await
            .map_err(anyhow::Error::msg)?,
    ))
}

pub(crate) async fn redis_health(
    State(state): State<AppState>,
    Path(connection_id): Path<String>,
) -> ApiResult<Json<bool>> {
    Ok(Json(
        connection_service::start_health_check_service(
            &state.redis_connection(&connection_id)?.pool,
        )
        .await?,
    ))
}

#[derive(Deserialize)]
pub(crate) struct RedisUrlRequest {
    redis_url: String,
}
pub(crate) async fn redis_version(
    Json(request): Json<RedisVersionRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let connection = CreateConnection {
        name: String::new(),
        host: request.host,
        port: request.port,
        username: request.username,
        password: request.password,
        db: Some(request.db),
        bullmq_prefix: None,
        color: None,
        label: None,
        is_tls_enabled: request.is_tls_enabled,
    };
    connection_service::check_redis_version(&connection).await?;
    Ok(Json(serde_json::json!({"supported": true})))
}

#[derive(Deserialize)]
pub(crate) struct RedisVersionRequest {
    host: String,
    port: i32,
    username: Option<String>,
    password: Option<String>,
    db: i32,
    is_tls_enabled: bool,
}
