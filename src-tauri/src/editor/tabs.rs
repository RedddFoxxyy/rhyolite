//! This module provides document tabs related commands for the app.
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

use crate::app_state::{AppState, CommandRegistrar, CommandRegistry, Tab};
use crate::commands::event_emitter;

use crate::utils::generate_available_path;
use crate::FileInfo;
use std::sync::{Arc, Mutex};

use super::io::{get_trove_dir, save_document, save_user_data};

pub struct TabCommands;

impl TabCommands {
    pub fn new_tab(app: AppHandle, _payload: Option<String>) {
        log::debug!("new_tab init");
        let temp_app = app.clone();
        let state = &temp_app.state::<AppState>();

        let new_id = Uuid::new_v4().to_string();

        let trove_dir = get_trove_dir("Untitled_Trove");

        let new_path = generate_available_path(trove_dir.join("Untitled.md"));
        let title = new_path.file_stem().unwrap().to_string_lossy().to_string();

        cleanup_deleted_files_workaround(state, trove_dir, &title);

        // Create new tab
        let new_tab = Tab {
            id: new_id.clone(),
            title: title.clone(),
        };

        // Insert into IndexMap
        {
            let mut tab_switcher = state.tab_switcher.write().unwrap();
            tab_switcher.tabs.insert(new_id.clone(), new_tab.clone());
            tab_switcher.current_tab_id = Some(new_id.clone());
        }

        {
            state
                .workspace
                .write()
                .unwrap()
                .recent_files
                .push(FileInfo {
                    id: new_id.clone(),
                    title: title.clone(),
                });
        }

        let _ = save_user_data(state);
        let _ = save_document(new_id, title, String::new(), state.to_owned());
        event_emitter(app);
    }

    pub fn close_tab(app: AppHandle, payload: Option<String>) {
        log::debug!("close_tab init");
        let Some(payload) = payload else {
            log::warn!("Invalid call to close_tab");
            return;
        };

        // Parse the JSON payload
        let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&payload) else {
            log::debug!("Failed to parse JSON payload");
            return;
        };
        let Some(tab_id) = json_value.get("tabId").and_then(|v| v.as_str()) else {
            log::debug!("Invalid JSON payload format: missing or invalid tabId field");
            return;
        };
        let temp_app = app.clone();
        let state = temp_app.state::<AppState>();

        // Get next tab ID in a separate scope to minimize lock time
        let next_tab_id = {
            let tab_switcher = state.tab_switcher.read().unwrap();
            let tabs = &tab_switcher.tabs;

            if tabs.is_empty() {
                return; // Don't close the last tab
            }

            if let Some((index, _, _)) = tabs.get_full(tab_id) {
                // Get the next tab ID (either at same index or last tab)
                tabs.get_index(index + 1)
                    .or_else(|| tabs.last())
                    .map(|(id, _)| id.clone())
            } else {
                None
            }
        };

        // Update tabs in a separate lock scope
        {
            let mut tab_switcher = state.tab_switcher.write().unwrap();
            tab_switcher.tabs.shift_remove(tab_id);

            // Update current open tab if needed
            if let Some(next_id) = next_tab_id {
                tab_switcher.current_tab_id = Some(next_id);
            }
        }

        // Call event emitter after releasing the lock
        event_emitter(app);
    }

    pub fn update_states(app: AppHandle, _payload: Option<String>) {
        event_emitter(app);
    }

    pub fn update_tab_title(app: AppHandle, payload: Option<String>) {
        log::debug!("update_tab_title init");
        let Some(payload) = payload else {
            log::warn!("Invalid call to update_tab_title");
            return;
        };

        if let Ok(payload_object) = serde_json::from_str::<Tab>(&payload) {
            let temp_app = app.clone();
            let state = temp_app.state::<AppState>();

            let tabs = &mut state.tab_switcher.write().unwrap().tabs;
            let title = payload_object.title;
            let id = payload_object.id;

            // Check if the new title already exists in other tabs
            if tabs.values().any(|tab| tab.id != id && tab.title == title) {
                log::debug!("A tab with this title already exists.");
            } else {
                // Get the tab, update its title, and insert it back
                if let Some(mut tab) = tabs.get(&id).cloned() {
                    tab.title = title;
                    tabs.insert(id, tab.clone());
                } else {
                    log::debug!("Tab not found");
                }
            }
        }

        event_emitter(app);
    }

    pub fn switch_tab(app: AppHandle, payload: Option<String>) {
        let Some(payload) = payload else {
            log::warn!("Invalid call to switch_tab");
            return;
        };
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&payload) {
            if let Some(tab_id) = json_value.get("tabId").and_then(|v| v.as_str()) {
                let temp_app = app.clone();
                let state = temp_app.state::<AppState>();

                let tab_switcher = &mut state.tab_switcher.write().unwrap();

                if tab_switcher.tabs.values().any(|tab| tab.id == tab_id) {
                    // Update current open tab if needed
                    tab_switcher.current_tab_id = Some(tab_id.to_string());
                }
            }
        }
        event_emitter(app);
    }

    pub fn load_tab(app: AppHandle, payload: Option<String>) {
        log::debug!("load_tab init");
        let Some(payload) = payload else {
            log::warn!("Invalid call to load_tab");
            return;
        };
        if let Ok(json_value) = serde_json::from_str::<Tab>(&payload) {
            {
                let temp_app = app.clone();
                let state = temp_app.state::<AppState>();
                let id = json_value.id;
                let title = json_value.title;
                let new_tab = Tab {
                    id: id.clone(),
                    title,
                };

                let mut tab_switcher = state.tab_switcher.write().unwrap();
                tab_switcher.tabs.insert(id, new_tab.clone());
            }

            event_emitter(app);
        }
    }
}

fn cleanup_deleted_files_workaround(
    state: &State<'_, crate::app_state::AppStateInner>,
    trove_dir: std::path::PathBuf,
    title: &String,
) {
    // Clean up any stale entries in tabs and recent_files that don't exist on disk
    // but have the same title
    {
        state.tab_switcher.write().unwrap().tabs.retain(|_, tab| {
            let file_path =
                trove_dir.join(sanitize_filename::sanitize(format!("{}.md", &tab.title)));
            file_path.exists() && tab.title != *title
        });
    }
    {
        state
            .workspace
            .write()
            .unwrap()
            .recent_files
            .retain(|file| {
                let file_path =
                    trove_dir.join(sanitize_filename::sanitize(format!("{}.md", &file.title)));
                file_path.exists() && file.title != *title
            });
    }
}

impl CommandRegistrar for TabCommands {
    fn register_commands(registry: &mut CommandRegistry) {
        // Register the methods directly
        registry.add_command(
            "new_tab".to_string(),
            Arc::new(Mutex::new(Box::new(Self::new_tab))),
        );
        registry.add_command(
            "close_tab".to_string(),
            Arc::new(Mutex::new(Box::new(Self::close_tab))),
        );
        registry.add_command(
            "update_states".to_string(),
            Arc::new(Mutex::new(Box::new(Self::update_states))),
        );
        registry.add_command(
            "update_tab_title".to_string(),
            Arc::new(Mutex::new(Box::new(Self::update_tab_title))),
        );
        registry.add_command(
            "switch_tab".to_string(),
            Arc::new(Mutex::new(Box::new(Self::switch_tab))),
        );
        registry.add_command(
            "load_tab".to_string(),
            Arc::new(Mutex::new(Box::new(Self::load_tab))),
        );
    }
}

#[tauri::command]
pub fn send_current_open_tab(id: String, state: State<'_, AppState>) {
    log::debug!("send_current_open_tab init");
    state.tab_switcher.write().unwrap().current_tab_id = Some(id.clone());
}

#[tauri::command]
pub fn get_current_open_tab(state: State<'_, AppState>) -> Result<String, String> {
    log::debug!("get_current_open_tab init");
    return state
        .tab_switcher
        .read()
        .unwrap()
        .current_tab_id
        .clone()
        .ok_or("Tab doesn't exist".to_string());
}

#[tauri::command]
pub fn get_tabs(state: State<'_, AppState>) -> Result<Vec<Tab>, String> {
    log::debug!("get_tabs init");
    Ok(state
        .tab_switcher
        .read()
        .unwrap()
        .tabs
        .values()
        .cloned()
        .collect())
}

#[tauri::command]
pub fn update_states(app: AppHandle) {
    event_emitter(app);
}

#[tauri::command]
pub fn new_tab(app: AppHandle) -> Result<Tab, String> {
    log::debug!("new_tab init");
    let temp_app = app.clone();
    let state = temp_app.state::<AppState>();
    let orig_state = &state;

    let new_id = Uuid::new_v4().to_string();

    let trove_dir = get_trove_dir("Untitled_Trove");
    let new_path = generate_available_path(trove_dir.join("Untitled.md"));
    let title = new_path.file_stem().unwrap().to_string_lossy().to_string();

    cleanup_deleted_files_workaround(&state, trove_dir, &title);

    // Create new tab
    let new_tab = Tab {
        id: new_id.clone(),
        title: title.clone(),
    };

    // Insert into IndexMap
    {
        let mut tab_switcher = state.tab_switcher.write().unwrap();
        tab_switcher.tabs.insert(new_id.clone(), new_tab.clone());
        tab_switcher.current_tab_id = Some(new_id.clone());
    }

    {
        state
            .workspace
            .write()
            .unwrap()
            .recent_files
            .push(FileInfo {
                id: new_id.clone(),
                // FIXME: hardcoded name may have conflict
                title: "Untitled".to_string(),
            });
    }

    save_user_data(orig_state)?;
    let _ = save_document(new_id, title, String::new(), orig_state.to_owned());
    event_emitter(app);

    Ok(new_tab)
}

#[tauri::command]
pub fn load_tab(
    app: AppHandle,
    id: String,
    title: String,
    state: State<'_, AppState>,
) -> Result<Tab, String> {
    log::debug!("load_tab init");

    let new_tab = Tab {
        id: id.clone(),
        title,
    };

    {
        let mut tab_switcher = state.tab_switcher.write().unwrap();
        tab_switcher.tabs.insert(id, new_tab.clone());
    }

    event_emitter(app);

    Ok(new_tab)
}
