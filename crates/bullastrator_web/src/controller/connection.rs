use crate::models::{connection::RedisVersionRequest, user::AuthenticatedUser};
use crate::{controller::ApiResult, models::connection::RedisUrlRequest};
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use bullastrator_core::{services::connection as connection_service, state::AppState};
use bullastrator_storage::models::{Connection, CreateConnection, UpdateConnection};

pub(crate) async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(request): Json<CreateConnection>,
) -> ApiResult<Json<Connection>> {
    Ok(Json(state.connections.create(&user.id, request).await?))
}

pub(crate) async fn list(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> ApiResult<Json<Vec<Connection>>> {
    Ok(Json(state.connections.list(&user.id).await?))
}

pub(crate) async fn get(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(connection_id): Path<String>,
) -> ApiResult<Json<Option<Connection>>> {
    Ok(Json(state.connections.get(&connection_id, &user.id).await?))
}

pub(crate) async fn update(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(connection_id): Path<String>,
    Json(request): Json<UpdateConnection>,
) -> ApiResult<Json<Connection>> {
    Ok(Json(
        state
            .connections
            .update(&connection_id, &user.id, request)
            .await?,
    ))
}

pub(crate) async fn delete(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(connection_id): Path<String>,
) -> ApiResult<Json<u64>> {
    Ok(Json(
        state.connections.delete(&connection_id, &user.id).await?,
    ))
}

pub(crate) async fn test_redis(Json(request): Json<RedisUrlRequest>) -> ApiResult<Json<bool>> {
    Ok(Json(
        connection_service::test_redis_connection_service(request.redis_url)
            .await
            .map_err(anyhow::Error::msg)?,
    ))
}

pub(crate) async fn redis_health(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(connection_id): Path<String>,
) -> ApiResult<Json<bool>> {
    state
        .connections
        .authorize(
            &connection_id,
            &user.id,
            bullastrator_storage::models::WorkspaceRole::VIEWER,
        )
        .await?;
    Ok(Json(
        connection_service::start_health_check_service(
            &state.get_redis_connection(&connection_id).await?.pool,
        )
        .await?,
    ))
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
