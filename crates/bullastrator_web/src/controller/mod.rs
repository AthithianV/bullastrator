pub(crate) mod connection;
pub(crate) mod job;
pub(crate) mod queue;
pub(crate) mod user;
pub(crate) mod workspace;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub(crate) async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok"}))
}

pub(crate) type ApiResult<T> = std::result::Result<T, ApiError>;

pub(crate) struct ApiError(anyhow::Error);

impl From<anyhow::Error> for ApiError {
    fn from(error: anyhow::Error) -> Self {
        Self(error)
    }
}

impl From<bullastrator_core::error::BullastratorError> for ApiError {
    fn from(error: bullastrator_core::error::BullastratorError) -> Self {
        Self(anyhow::Error::new(error))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": self.0.to_string()})),
        )
            .into_response()
    }
}
