use crate::{
    app_state::{DocumentData, Tab, TROVE_DIR},
    editor::{
        io::{get_trove_dir, IOCommands},
        markdown_handler,
    },
};
use std::fs;
use tauri::{AppHandle, Emitter};

impl IOCommands {
    //TODO: Cleanup unused variables.
    pub fn get_document_content(app: AppHandle, payload: Option<String>) {
        let Some(payload) = payload else {
            log::warn!("Invalid call to get_document_content");
            return;
        };

        if let Ok(tab_data) = serde_json::from_str::<Tab>(&payload) {
            let document_data = get_document_content_helper(tab_data);
            if document_data.is_none() {
                log::error!("Failed to get document data!");
                return;
            }
            let html_output = markdown_handler::markdown_to_html(&document_data.unwrap().content);

            // Update the current content on the screen.
            let _ = app.emit("current_editor_content", html_output);
        }
    }
}

pub fn get_document_content_helper(tab_data: Tab) -> Option<DocumentData> {
    // let id = tab_data.id;
    let title = tab_data.title;

    // Get the path of the document using title
    let trove_dir = get_trove_dir(TROVE_DIR);
    let file_path = trove_dir.join(format!("{}.md", title));

    // Check if the file exists
    if !file_path.exists() {
        // If the file does not exist, return None
        log::error!("File not found!");
        return None;
    }

    // Read the file content using the file path
    match fs::read_to_string(&file_path) {
        // If the file is read successfully, convert the markdown content to HTML
        Ok(content) => {
            // let html_output = markdown_handler::markdown_to_html(&content);

            // // Update the current content on the screen.
            // let _ = app.emit("current_editor_content", html_output);
            let documentdata = DocumentData {
                id: tab_data.id,
                title,
                content,
            };
            Some(documentdata)
        } // If there is an error in reading the file, return the error
        Err(_e) => None,
    }
}
