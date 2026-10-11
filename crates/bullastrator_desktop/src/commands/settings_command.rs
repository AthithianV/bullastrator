// src-tauri/src/lib.rs
use tauri::WebviewWindow;

#[tauri::command]
pub fn set_app_zoom(window: WebviewWindow, scale: f64) {
    // In Tauri v2, we access the webview through the window handle
    let _ = window.set_zoom(scale).map_err(|e| e.to_string());
}

#[tauri::command]
pub fn show_main_window(window: tauri::Window) {
    window.show().unwrap();
    window.set_focus().unwrap();
}
