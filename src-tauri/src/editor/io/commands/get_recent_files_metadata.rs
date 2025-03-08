use crate::{
    app_state::{AppState, UserData},
    editor::io::{IOCommands, get_documents_dir, save_user_data},
};
use std::fs;
use tauri::{AppHandle, Emitter, Manager};

impl IOCommands {
    pub async fn get_recent_files_metadata(app: AppHandle, _payload: Option<String>) {
        let temp_app = app.clone();
        let state = &temp_app.state::<AppState>();
        if let Err(e) = save_user_data(state).await {
            log::error!("Warning: Failed to save user data: {}", e);
        }
        let appdata_dir = get_documents_dir().join("appdata");
        let userdata_path = appdata_dir.join("userdata.json");

        let metadata = if userdata_path.exists() {
            let content_result = fs::read_to_string(&userdata_path);
            if let Err(e) = &content_result {
                log::error!(
                    "Failed to get the metadata content from the userdata path: {}",
                    e
                );
            }
            let content = content_result.unwrap();

            // Try to deserialize the JSON.
            let user_data_result = serde_json::from_str::<UserData>(&content);
            if let Err(e) = &user_data_result {
                log::error!(
                    "Failed to get the user_data from the metadata content: {}",
                    e
                );
            }

            user_data_result.unwrap().recent_files
        } else {
            log::error!("Failed to get recent files metadata, userdata_path does not exist!");
            Vec::new()
        };

        if let Err(emit_err) = app.emit("recent_files_metadata", metadata) {
            log::error!("Failed to send the recent files metadata: {:#?}", emit_err);
        }
    }
}
