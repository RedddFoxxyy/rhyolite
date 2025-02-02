use app_state::{AppStateInner, RecentFileInfo};
use tauri::{Manager, WindowEvent};
mod app_state;
mod commands;
mod editor;

//Main tauri function.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .setup(|app| {
            app.manage(AppStateInner::load().expect("Failed to load config"));
            crate::commands::load_default_commands(app.app_handle().to_owned());
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                // Call the function to save UserData when the app is closing
                editor::io::on_app_close(window);

                // Prevent the window from closing immediately
                #[cfg(not(target_os = "android"))]
                window.close().unwrap();
            }
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            editor::io::save_document,
            editor::io::load_last_open_tabs,
            editor::io::delete_document,
            editor::io::get_document_content,
            editor::io::get_recent_files_metadata,
            editor::tabs::update_states,
            editor::tabs::new_tab,
            editor::tabs::load_tab,
            editor::tabs::get_tabs,
            editor::tabs::send_current_open_tab,
            editor::tabs::get_current_open_tab,
            // editor::tabs::update_tab_title,
            // editor::tabs::close_tab,
            commands::exec_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
