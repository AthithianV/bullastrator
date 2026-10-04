use crate::{controller::ApiResult, server::AppState};
use axum::{
    extract::{Path, State},
    Json,
};
use bullastrator_core::services::queue::QueueDetails;
use serde::Deserialize;

pub(crate) async fn all_queues(State(state): State<AppState>) -> ApiResult<Json<Vec<String>>> {
    Ok(Json(state.queues.get_all_bullmq_queues().await?))
}

pub(crate) async fn queue_details(
    State(state): State<AppState>,
    Path(queue): Path<String>,
) -> ApiResult<Json<QueueDetails>> {
    Ok(Json(state.queues.get_queue_details_service(&queue).await?))
}

#[derive(Deserialize)]
pub(crate) struct PauseRequest {
    paused: bool,
}
pub(crate) async fn pause_queue(
    State(state): State<AppState>,
    Path(queue): Path<String>,
    Json(request): Json<PauseRequest>,
) -> ApiResult<Json<String>> {
    Ok(Json(
        state
            .queues
            .pause_queue_service(&queue, request.paused)
            .await?,
    ))
}
