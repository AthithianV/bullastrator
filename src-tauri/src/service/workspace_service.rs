use anyhow::Result;
use chrono::Utc;
use tauri::{AppHandle, Manager, State};

use crate::{
    AppState,
    model::{
        user_model::ReadUserModel,
        workspace_model::{
            ApiWorkspaceModel, CreateWorkspaceModel, ReadWorkspaceModel, UpdateWorkspaceModel,
        },
    },
    repository::workspace_repository::WorkspaceRepository,
};

pub async fn initialize_user_workspaces(
    app: &AppHandle,
    user: &ReadUserModel,
    api_workspaces: Vec<ApiWorkspaceModel>,
) -> Result<()> {
    let state: State<'_, AppState> = app.state();
    let db_conn = state
        .get_app_db_connection()
        .await
        .map_err(|e| anyhow::anyhow!(e))?;

    let workspace_repo = WorkspaceRepository::new(&db_conn);

    let active_workspace = if !api_workspaces.is_empty() {
        // 1. If API provides workspaces, upsert them and find the primary one
        let mut primary_ws = None;
        for api_ws in api_workspaces {
            let create_model = CreateWorkspaceModel {
                name: api_ws.name.clone(),
                user_id: Some(user.id.clone()),
                plan: user.plan.clone(), // Use user's plan for the workspace
                role: api_ws.role.clone(),
                max_connections: api_ws.max_connection,
                icon: None,
                color: Some("#00CADB".to_string()),
                last_accessed_at: Utc::now(),
            };
            let upserted = workspace_repo.upsert(create_model, api_ws.id).await?;

            // Update is_primary in local DB
            let _ = workspace_repo
                .update(
                    upserted.id,
                    UpdateWorkspaceModel {
                        is_primary: Some(api_ws.is_primary),
                        ..Default::default()
                    },
                )
                .await?;

            if api_ws.is_primary {
                primary_ws = Some(ReadWorkspaceModel {
                    is_primary: true,
                    ..upserted
                });
            }
        }
        primary_ws.unwrap_or_else(|| {
            // Fallback to first if no primary found
            unreachable!("API should provide at least one primary workspace")
        })
    } else {
        // 2. Fallback: Search for existing workspaces for this User ID or create default
        let user_workspaces = workspace_repo.get_by_user_id(&user.id).await?;

        if let Some(ws) = user_workspaces.first() {
            if ws.plan != "FREE" {
                workspace_repo
                    .update(
                        ws.id,
                        UpdateWorkspaceModel {
                            is_guest_mode: Some(false),
                            ..Default::default()
                        },
                    )
                    .await?;
            }
            ReadWorkspaceModel {
                is_guest_mode: user.plan == "FREE",
                ..ws.clone()
            }
        } else {
            let create_model = CreateWorkspaceModel {
                name: "My Workspace".to_string(),
                user_id: Some(user.id.clone()),
                plan: "FREE".to_string(),
                role: "OWNER".to_string(),
                max_connections: 1,
                icon: None,
                color: Some("#00CADB".to_string()),
                last_accessed_at: Utc::now(),
            };
            workspace_repo.create(create_model).await?
        }
    };

    let existing_active_workspace = workspace_repo.get_active_workspace().await?;

    if existing_active_workspace.is_guest_mode {
        // 3. Set this as the active workspace in both DB and AppState
        workspace_repo
            .set_active_workspace(active_workspace.id)
            .await?;
        state.set_active_workspace(active_workspace.clone()).await;
    }

    Ok(())
}
