mod commands;

use std::sync::Mutex;

/// Application state, shared across every command via `State<'_, AppState>`.
/// Tauri hands out `&AppState`, so anything mutable needs interior mutability.
#[derive(Default)]
pub struct AppState {
    pub counter: Mutex<i64>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::app_info,
            commands::bump_counter,
            commands::slow_task,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
