use crate::controller::ApiResult;
use axum::{
    Json,
    extract::{Path, State},
};
use bullastrator_core::{services::queue::QueueDetails, state::AppState};
use serde::Deserialize;

pub(crate) async fn all_queues(
    State(state): State<AppState>,
    Path(connection_id): Path<String>,
) -> ApiResult<Json<Vec<String>>> {
    Ok(Json(
        state
            .queue_service(&connection_id)?
            .get_all_bullmq_queues()
            .await?,
    ))
}

pub(crate) async fn queue_details(
    State(state): State<AppState>,
    Path((connection_id, queue)): Path<(String, String)>,
) -> ApiResult<Json<QueueDetails>> {
    Ok(Json(
        state
            .queue_service(&connection_id)?
            .get_queue_details_service(&queue)
            .await?,
    ))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PauseRequest {
    paused: bool,
}
pub(crate) async fn pause_queue(
    State(state): State<AppState>,
    Path((connection_id, queue)): Path<(String, String)>,
    Json(request): Json<PauseRequest>,
) -> ApiResult<Json<String>> {
    Ok(Json(
        state
            .queue_service(&connection_id)?
            .pause_queue_service(&queue, request.paused)
            .await?,
    ))
}
