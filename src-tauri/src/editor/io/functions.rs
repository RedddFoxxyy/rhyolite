//TODO: We can name this file something better instead of naming it as functions.

use std::fs; //Filesystem module
use std::path::PathBuf;
use tauri::{Manager, State, Window};
use uuid::Uuid;
// use tauri_plugin_dialog::DialogExt; //DialogExt trait to show dialog boxes

use dirs;

use crate::{
    FileInfo,
    app_state::{AppState, CommandRegistrar, CommandRegistry, DocumentData, Tab, UserData},
    editor::{
        io::commands::get_document_content::get_document_content_helper, settings::themes::Theme,
    },
};

pub struct IOCommands;

impl CommandRegistrar for IOCommands {
    fn register_commands(registry: &mut CommandRegistry) {
        registry.add_command(
            "save_document".to_string(),
            Box::new(|app, payload| Box::pin(Self::save_document(app, payload))),
        );
        registry.add_command(
            "delete_document".to_string(),
            Box::new(|app, payload| Box::pin(Self::delete_document(app, payload))),
        );
        registry.add_command(
            "get_document_content".to_string(),
            Box::new(|app, payload| Box::pin(Self::get_document_content(app, payload))),
        );
        registry.add_command(
            "load_last_open_tabs".to_string(),
            Box::new(|app, payload| Box::pin(Self::load_last_open_tabs(app, payload))),
        );
        registry.add_command(
            "get_recent_files_metadata".to_string(),
            Box::new(|app, payload| Box::pin(Self::get_recent_files_metadata(app, payload))),
        );
    }
}

/// This function returns the path to the documents' directory.
pub fn get_documents_dir() -> PathBuf {
    #[cfg(target_os = "android")]
    {
        // On Android, use the app's private storage directory
        let path = PathBuf::from("/data/user/0/com.rhyolite.dev/Rhyolite");
        // Create the directory if it doesn't exist
        fs::create_dir_all(&path).expect("Could not create Rhyolite directory");
        path
    }

    #[cfg(not(target_os = "android"))]
    {
        // Original desktop behavior
        let mut path = dirs::document_dir().expect("Could not find Documents directory");
        // TODO: Use a const for this name.
        path.push("Rhyolite");
        // Create the directory if it doesn't exist
        fs::create_dir_all(&path).expect("Could not create Rhyolite directory");
        path
    }
}

/// This function returns the path to the default trove directory.
pub fn get_trove_dir(trove_name: &str) -> PathBuf {
    //Get the path to documents/Rhyolite.
    let documents_dir = get_documents_dir();

    //Append the default trove name to the 'documents/Rhyolite path'.
    let trove_dir = documents_dir.join(trove_name);

    //Then create the path 'documents/Rhyolite/trove_name' if it does not
    fs::create_dir_all(&trove_dir).expect("Could not create Trove directory");

    //retrun the path of the default trove directory.
    trove_dir
}

/// Runs when the app is closing and saves the user data.
pub fn on_app_close(window: &Window) {
    log::debug!("on_app_close init");
    let state = window.state::<AppState>();

    if let Err(err_saving) = save_user_data(&state) {
        log::error!(
            "Failed to save the workspace before closing: {}",
            err_saving
        );
    }
}

/// This function saves the user data to the userdata.json file.
pub fn save_user_data(state: &State<'_, AppState>) -> Result<(), String> {
    let user_data = {
        let maybe_tab_switcher = state.get_tab_switcher();
        let maybe_workspace = state.get_workspace();

        if maybe_tab_switcher.is_none() || maybe_workspace.is_none() {
            log::error!("Failed to save user data!");
            return Err("Failed to save user data!".to_string());
        }

        let tab_switcher = maybe_tab_switcher.unwrap();
        let workspace = maybe_workspace.unwrap();

        UserData {
            tabs: tab_switcher.tabs.values().cloned().collect(),
            last_open_tab: tab_switcher.current_tab_id.clone().unwrap(),
            recent_files: workspace.recent_files.clone(),
            current_theme: Theme::default(),
        }
    };

    let appdata_dir = get_documents_dir().join("appdata");
    fs::create_dir_all(&appdata_dir).expect("Could not create appdata directory");
    let userdata_path = appdata_dir.join("userdata.json");

    match serde_json::to_string_pretty(&user_data) {
        Ok(json_content) => fs::write(userdata_path, json_content)
            .map_err(|e| format!("Failed to save userdata: {}", e)),
        Err(e) => Err(format!("Failed to serialize userdata: {}", e)),
    }
}

/// This function loads the tabs active/opened in the last app section.
#[tauri::command]
pub fn load_last_open_tabs(state: State<'_, AppState>) -> Result<Vec<DocumentData>, String> {
    log::debug!("load_last_open_tabs init");
    let appdata_dir = get_documents_dir().join("appdata");
    let userdata_path = appdata_dir.join("userdata.json");

    if userdata_path.exists() {
        return match fs::read_to_string(&userdata_path) {
            Ok(content) => match serde_json::from_str::<UserData>(&content) {
                Ok(user_data) => {
                    let mut last_open_files = Vec::new();
                    // Update workspace in a separate scope
                    {
                        let maybe_workspace = state.get_workspace_mut();
                        let maybe_tab_switcher = state.get_tab_switcher_mut();
                        if maybe_workspace.is_none() || maybe_tab_switcher.is_none() {
                            return Err("Failed to load last open tabs".to_string());
                        }
                        let mut workspace = maybe_workspace.unwrap();
                        workspace.recent_files = user_data.recent_files.clone();

                        let mut tab_switcher = maybe_tab_switcher.unwrap();
                        tab_switcher.current_tab_id = Some(user_data.last_open_tab.clone());

                        // Clear existing tabs and load from user_data
                        let tabs = &mut tab_switcher.tabs;
                        tabs.clear();

                        // Process tabs and load documents
                        for tab in user_data.tabs {
                            match get_document_content_helper(tab.clone()) {
                                Some(doc) => {
                                    last_open_files.push(doc);

                                    tab_switcher.tabs.insert(tab.id.clone(), tab.clone());
                                }
                                _ => continue,
                            }
                        }
                    }

                    Ok(last_open_files)
                }
                Err(e) => Err(format!("Failed to deserialize userdata: {}", e)),
            },
            Err(e) => Err(format!("Failed to read userdata file: {}", e)),
        };
    }

    // If userdata.json doesn't exist, load all markdown files from the trove directory
    let trove_dir = get_trove_dir("Untitled_Trove");

    let files = match fs::read_dir(&trove_dir) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "md"))
            .filter_map(|entry| {
                let path = entry.path();
                let title = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .map(String::from)
                    .unwrap_or_default();

                let id = Uuid::new_v4().to_string();
                get_document_content_helper(Tab { id, title })
            })
            .collect(),
        Err(e) => return Err(format!("Failed to read directory: {}", e)),
    };

    Ok(files)
}

/// This function returns the metadata of the recent files.
#[tauri::command]
pub fn get_recent_files_metadata(state: State<'_, AppState>) -> Result<Vec<FileInfo>, String> {
    if let Err(e) = save_user_data(&state) {
        eprintln!("Warning: Failed to save user data: {}", e);
    }
    let appdata_dir = get_documents_dir().join("appdata");
    let userdata_path = appdata_dir.join("userdata.json");

    // Check if userdata.json exists
    if userdata_path.exists() {
        // Read and deserialize the UserData
        match fs::read_to_string(&userdata_path) {
            Ok(content) => match serde_json::from_str::<UserData>(&content) {
                Ok(user_data) => Ok(user_data.recent_files),
                Err(e) => Err(format!("Failed to deserialize userdata: {}", e)),
            },
            Err(e) => Err(format!("Failed to read userdata file: {}", e)),
        }
    } else {
        // If userdata.json doesn't exist, return empty vector
        Ok(Vec::new())
    }
}
