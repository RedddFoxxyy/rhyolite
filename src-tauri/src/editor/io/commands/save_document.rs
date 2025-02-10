use std::fs;
use tauri::{AppHandle, Manager};
use crate::app_state::{AppState, DocumentData, FileInfo};
use crate::editor::io::{get_trove_dir, IOCommands};
use crate::editor::markdown_handler;

impl IOCommands {
    ///BUG: The save_document command does not save the document
    ///when called from the frontend using exec_command.
    pub fn save_document(app: AppHandle, payload: Option<String>) {
        let Some(payload) = payload else {
            log::warn!("Invalid call to save_document");
            return;
        };
        log::debug!("save_document init");
        let temp_app = app.clone();
        let state = &temp_app.state::<AppState>();

        if let Ok(document_data) = serde_json::from_str::<DocumentData>(&payload) {
            {
                let mut workspace = state.workspace.write().unwrap();
                if let Some(doc) = workspace
                    .recent_files
                    .iter_mut()
                    .find(|doc| doc.id == document_data.id)
                {
                    doc.title = document_data.title.clone();
                } else {
                    workspace.recent_files.push(FileInfo {
                        id: document_data.id.clone(),
                        title: document_data.title.clone(),
                    });
                }
            }

            let trove_dir = get_trove_dir("Untitled_Trove");
            let markdown_content = markdown_handler::html_to_markdown(&document_data.content);
            let safe_filename = sanitize_filename::sanitize(format!("{}.md", document_data.title));
            let file_path = trove_dir.join(&safe_filename);

            // Get the old title in a separate scope
            let old_title = {
                let tab_switcher = state.tab_switcher.read().unwrap();
                tab_switcher
                    .tabs
                    .get(&document_data.id)
                    .map(|tab| tab.title.clone())
                    .unwrap_or_else(|| String::from("Untitled"))
            };

            let old_path = trove_dir.join(sanitize_filename::sanitize(format!("{}.md", old_title)));

            // if the title has changed, delete the old file
            if old_path != file_path && old_path.exists() {
                let _ = fs::remove_file(old_path)
                    .map_err(|e| format!("Failed to delete old file: {}", e));
            }

            let _ = if let Err(e) = fs::write(&file_path, markdown_content) {
                Err(format!("Failed to write file: {}", e))
            } else {
                Ok(file_path.to_string_lossy().to_string())
            };
        }
    }
}