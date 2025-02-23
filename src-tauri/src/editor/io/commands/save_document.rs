use crate::{
    app_state::{AppState, DocumentData, FileInfo, DEFAULT_NOTE_TITLE, TROVE_DIR},
    editor::{
        io::{get_trove_dir, IOCommands},
        markdown_handler,
    },
};
use std::fs;
use tauri::{AppHandle, Manager};

impl IOCommands {
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
                let workspace_option = state.get_workspace_mut();
                if workspace_option.is_none() {
                    log::error!("Failed to save document!");
                    return;
                }

                let mut workspace = workspace_option.unwrap();

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

            let trove_dir = get_trove_dir(TROVE_DIR);
            let markdown_content = markdown_handler::html_to_markdown(&document_data.content);
            let safe_filename = sanitize_filename::sanitize(format!("{}.md", document_data.title));
            let file_path = trove_dir.join(&safe_filename);

            // Get the old title in a separate scope
            let old_title = {
                let tab_switcher_option = state.get_tab_switcher();
                if tab_switcher_option.is_none() {
                    log::error!("Failed to save document!");
                    return;
                }
                tab_switcher_option
                    .unwrap()
                    .tabs
                    .get(&document_data.id)
                    .map(|tab| tab.title.clone())
                    .unwrap_or_else(|| String::from(DEFAULT_NOTE_TITLE))
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
