use crate::{
    controller::ApiResult,
    models::{queue::PauseRequest, user::AuthenticatedUser},
};
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use bullastrator_core::{services::queue::QueueDetails, state::AppState};

pub(crate) async fn sync_queues(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(connection_id): Path<String>,
) -> ApiResult<Json<Vec<String>>> {
    let queue_service = state.get_queue_service(&connection_id).await?;

    Ok(Json(queue_service.sync_queues(&user.id).await?))
}

pub(crate) async fn queue_details(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, queue)): Path<(String, String)>,
) -> ApiResult<Json<QueueDetails>> {
    let queue_service = state.get_queue_service(&connection_id).await?;

    Ok(Json(
        queue_service
            .get_queue_details_service(&queue, &user.id)
            .await?,
    ))
}

pub(crate) async fn pause_queue(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path((connection_id, queue)): Path<(String, String)>,
    Json(request): Json<PauseRequest>,
) -> ApiResult<Json<String>> {
    let queue_service = state.get_queue_service(&connection_id).await?;

    Ok(Json(
        queue_service
            .pause_queue_service(&queue, request.paused, &user.id)
            .await?,
    ))
}
