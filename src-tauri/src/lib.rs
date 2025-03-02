//! # Rhyolite Library
//! This is the entry point for the application backend
//! and this is where the whole app is first initialised and
//! runs.

// NOTE: I need help with improving code documentation and comments
// PRs are welcomed with regard to docs/comments! Thank You.

use crate::editor::io;
use crate::editor::tabs;
use app_state::{AppStateInner, FileInfo};
use tauri::{Manager, WindowEvent};

mod app_state;
mod commands;
mod editor;
mod helpers;
mod utils;

//Main tauri function.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .setup(|app| {
            app.manage(AppStateInner::load().expect("Failed to load config"));
            commands::load_default_commands(app.app_handle());
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                // Call the function to save UserData when the app is closing
                io::on_app_close(window);

                // Prevent the window from closing immediately
                #[cfg(not(target_os = "android"))]
                window.close().unwrap();
            }
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            io::load_last_open_tabs,
            io::get_recent_files_metadata,
            tabs::update_states,
            tabs::new_tab,
            tabs::load_tab,
            tabs::get_tabs,
            tabs::send_current_open_tab,
            tabs::get_current_open_tab,
            commands::exec_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
