use crate::{
    app_state::{AppState, AppStateInner, DocumentData, FileInfo, DEFAULT_NOTE_TITLE, TROVE_DIR},
    editor::{
        io::{get_trove_dir, IOCommands},
        markdown_handler,
    },
};
use std::fs;
use tauri::{AppHandle, Manager, State};

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
            save_document_helper(state, document_data);
        }
    }
}

pub fn save_document_helper(state: &State<'_, AppStateInner>, document_data: DocumentData) {
    let trove_dir = get_trove_dir(TROVE_DIR);
    let markdown_content = markdown_handler::html_to_markdown(&document_data.content);
    let safe_filename = sanitize_filename::sanitize(format!("{}.md", document_data.title));
    let file_path = trove_dir.join(&safe_filename);

    {
        let maybe_workspace = state.get_workspace_mut();
        if maybe_workspace.is_none() {
            log::error!("Failed to save document!");
            return;
        }

        let mut workspace = maybe_workspace.unwrap();

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
                path: file_path.clone(),
            });
        }
    }

    // Get the old title in a separate scope
    let old_title = {
        let maybe_tab_switcher = state.get_tab_switcher();
        if maybe_tab_switcher.is_none() {
            log::error!("Failed to save document!");
            return;
        }
        maybe_tab_switcher
            .unwrap()
            .tabs
            .get(&document_data.id)
            .map(|tab| tab.title.clone())
            .unwrap_or_else(|| String::from(DEFAULT_NOTE_TITLE))
    };

    let old_path = trove_dir.join(sanitize_filename::sanitize(format!("{}.md", old_title)));

    // if the title has changed, delete the old file
    if old_path != file_path && old_path.exists() {
        let _ = fs::remove_file(old_path).map_err(|e| format!("Failed to delete old file: {}", e));
    }

    let _ = if let Err(e) = fs::write(&file_path, markdown_content) {
        Err(format!("Failed to write file: {}", e))
    } else {
        Ok(file_path.to_string_lossy().to_string())
    };
}
