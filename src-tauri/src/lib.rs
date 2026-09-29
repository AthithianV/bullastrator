pub mod bull_drivers;
pub mod commands;
pub mod entity;
pub mod model;
pub mod repository;
pub mod scripts;
pub mod service;
pub mod utilities;

use tauri::{Emitter, Manager, State, WindowEvent};
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_log::{Target, TargetKind};

pub use commands::connection_command::*;
pub use commands::folder_command::*;
pub use commands::job_command::*;
pub use commands::queue_command::*;
pub use commands::settings_command::*;
pub use commands::tab_command::*;
pub use commands::workspace_command::*;
pub use utilities::{app_state::AppState, local_db::initialize_local_db, logging::init_logging};

use crate::repository::workspace_repository::WorkspaceRepository;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();

    let app_state = AppState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .on_window_event(|window, event| {
            if let WindowEvent::Focused(true) = event {
                let _ = window.emit("resume-focus", ());
            }
        })
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();

            for arg in args {
                if arg.starts_with("bullastrator://") {
                    // Logic to process the URL (Refactor this into a helper function)
                    println!("Deep link ARG: {}", arg);
                }
            }
        }))
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(Target::new(TargetKind::LogDir {
                    file_name: Some("logs".to_string()),
                }))
                .max_file_size(50_000 /* bytes */)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .format(|out, message, record| {
                    out.finish(format_args!("\n[{}] {}\n", record.level(), message))
                })
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .max_file_size(50_000 /* bytes */)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    },
                ))
                .build(),
        )
        .manage(app_state.clone())
        .setup(|app| {
            init_logging();

            let handle_for_deep_link = app.handle().clone();
            let handle_for_db = app.handle().clone();

            #[cfg(target_os = "macos")]
            {
                use tauri::{LogicalPosition, TitleBarStyle, WebviewUrl, WebviewWindowBuilder};
                // macOS: native titlebar
                WebviewWindowBuilder::new(
                    &handle_for_db,
                    "main",
                    WebviewUrl::App("index.html".into()),
                )
                .title("")
                .decorations(true)
                .title_bar_style(TitleBarStyle::Overlay)
                .hidden_title(true)
                .traffic_light_position(LogicalPosition::new(16.0, 20.0))
                .maximized(true)
                .visible(false)
                .build()?;
            }

            #[cfg(not(target_os = "macos"))]
            {
                // Windows/Linux: custom titlebar
                use tauri::{WebviewUrl, WebviewWindowBuilder};
                WebviewWindowBuilder::new(
                    &handle_for_db,
                    "main",
                    WebviewUrl::App("index.html".into()),
                )
                .title("Bullastrator")
                .decorations(false)
                .transparent(false)
                .maximized(true)
                .visible(false)
                .build()?;
            }

            // let handle_clone = handle_for_deep_link.clone();
            handle_for_deep_link.deep_link().on_open_url(move |event| {
                for url in event.urls() {
                    println!("Deep link URL: {}", url);
                }
            });

            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 && args[1].starts_with("bullastrator://") {
                println!("Deep link ARG: {}", args[1]);
            }

            // spawn async task to initialize AppState
            tauri::async_runtime::spawn(async move {
                match initialize_local_db().await {
                    Ok(db_conn) => {
                        let state: State<'_, AppState> = handle_for_db.state();

                        // 1. Initialize DB Connection in State
                        state.set_app_db_connection(db_conn.clone()).await;

                        // 2. Initialize Workspace
                        let workspace_repository = WorkspaceRepository::new(&db_conn);
                        if let Ok(active_ws) = workspace_repository.get_active_workspace().await {
                            state.set_active_workspace(active_ws).await;
                        }
                    }
                    Err(e) => println!("Failed to initialize DB: {:?}", e),
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Connection Commands
            create_connection,
            update_connection,
            delete_connection,
            get_connection,
            get_all_connections,
            start_health_check,
            check_health_for_all_connections,
            test_redis_connection,
            // Workspace Commands
            create_workspace,
            get_all_workspaces,
            get_workspace_by_id,
            get_active_workspace,
            set_active_workspace,
            update_workspace,
            delete_workspace,
            // Queue Commands
            sync_all_queue_names,
            get_all_queues_by_connection,
            get_all_queues_by_workspace,
            pause_queue,
            get_queue_details,
            // Tabs Commands
            create_tab,
            update_tab,
            delete_tab,
            get_tab,
            get_all_tabs,
            reorder_tabs,
            set_active_tab,
            get_active_tab,
            get_jobs_data,
            // Job Commands
            get_jobs_in_queue,
            get_job_count_in_queue,
            add_job_to_queue,
            update_job_data,
            retry_failed_jobs,
            retry_all_failed_jobs,
            promote_jobs,
            promote_all_jobs,
            delete_jobs,
            delete_all_jobs,
            get_jobs_by_id,
            get_job_logs,
            // Folder Commands
            create_folder,
            get_all_folders,
            get_folder_by_id,
            update_folder,
            delete_folder,
            toggle_queue_in_folder,
            reorder_folder_queues,
            get_queues_for_folder,
            // Settings Commands
            set_app_zoom,
            show_main_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
