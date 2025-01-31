//! This module provides document tabs related functions for the app.
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;

use crate::app_state::{AppState, CommandRegistrar, CommandRegistry, Tab};
use crate::commands::event_emitter;

use crate::RecentFileInfo;
use std::path::Path;

use super::io::{get_trove_dir, save_document, save_user_data};

#[derive(serde::Deserialize)]
struct TabTitle {
    id: String,
    title: String,
}

pub struct TabCommands;
impl TabCommands {
    pub fn new_tab(app: AppHandle, _payload: String) {
        log::debug!("new_tab init");
        let temp_app = app.clone();
        let state = temp_app.state::<AppState>();
        let orig_state = &state;

        let new_id = Uuid::new_v4().to_string();

        let trove_dir = get_trove_dir("Untitled_Trove");

        let title = TabCommands.check_path_exists(&trove_dir);

        // Clean up any stale entries in tabs and recent_files that don't exist on disk
        // but have the same title
        {
            state.tab_switcher.lock().unwrap().tabs.retain(|_, tab| {
                let file_path =
                    trove_dir.join(sanitize_filename::sanitize(format!("{}.md", &tab.title)));
                file_path.exists() && tab.title != title
            });
        }
        {
            state.workspace.lock().unwrap().recent_files.retain(|file| {
                let file_path =
                    trove_dir.join(sanitize_filename::sanitize(format!("{}.md", &file.title)));
                file_path.exists() && file.title != title
            });
        }

        // Create new tab
        let new_tab = Tab {
            id: new_id.clone(),
            title: title.clone(),
        };

        // Insert into IndexMap
        {
            let mut tab_switcher = state.tab_switcher.lock().unwrap();
            tab_switcher.tabs.insert(new_id.clone(), new_tab.clone());
            tab_switcher.current_tab_id = Some(new_id.clone());
        }

        {
            state
                .workspace
                .lock()
                .unwrap()
                .recent_files
                .push(RecentFileInfo {
                    id: new_id.clone(),
                    // FIXME: hardcoded name may have conflict
                    title: "Untitled".to_string(),
                });
        }

        let _ = save_user_data(orig_state);
        let _ = save_document(new_id, title, String::new(), orig_state.to_owned());
        event_emitter(app);
    }

    fn check_path_exists(&self, trove_dir: &Path) -> String {
        let mut iteration: u32 = 0;
        loop {
            let title = if iteration == 0 {
                sanitize_filename::sanitize("Untitled.md")
            } else {
                sanitize_filename::sanitize(format!("Untitled {}.md", &iteration))
            };

            let file_path = trove_dir.join(&title);
            if !file_path.exists() {
                return title.strip_suffix(".md").unwrap_or(&title).to_string();
            }
            iteration += 1;
        }
    }

    pub fn close_tab(app: AppHandle, payload: String) {
        log::debug!("close_tab init");

        // Parse the JSON payload
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&payload) {
            if let Some(tab_id) = json_value.get("tabId").and_then(|v| v.as_str()) {
                let temp_app = app.clone();
                let state = temp_app.state::<AppState>();

                // Get next tab ID in a separate scope to minimize lock time
                let next_tab_id = {
                    let tab_switcher = state.tab_switcher.lock().unwrap();
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
                    let mut tab_switcher = state.tab_switcher.lock().unwrap();
                    tab_switcher.tabs.shift_remove(tab_id);

                    // Update current open tab if needed
                    if let Some(next_id) = next_tab_id {
                        tab_switcher.current_tab_id = Some(next_id);
                    }
                }

                // Call event emitter after releasing the lock
                event_emitter(app);
            } else {
                log::debug!("Invalid JSON payload format: missing or invalid tabId field");
            }
        } else {
            log::debug!("Failed to parse JSON payload");
        }
    }

    pub fn update_states(app: AppHandle, _payload: String) {
        event_emitter(app);
    }

    pub fn update_tab_title(app: AppHandle, payload: String) {
        log::debug!("update_tab_title init");

        if let Ok(payload_object) = serde_json::from_str::<TabTitle>(&payload) {
            let temp_app = app.clone();
            let state = temp_app.state::<AppState>();

            let tabs = &mut state.tab_switcher.lock().unwrap().tabs;
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
    pub fn switch_tab(app: AppHandle, payload: String) {
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&payload) {
            if let Some(tab_id) = json_value.get("tabId").and_then(|v| v.as_str()) {
                let temp_app = app.clone();
                let state = temp_app.state::<AppState>();

                let tab_switcher = &mut state.tab_switcher.lock().unwrap();

                if tab_switcher.tabs.values().any(|tab| tab.id == tab_id) {
                    // Update current open tab if needed
                    tab_switcher.current_tab_id = Some(tab_id.to_string());
                }
            }
        }
        event_emitter(app);
    }
}

impl CommandRegistrar for TabCommands {
    fn register_commands(registry: &mut CommandRegistry) {
        // Register the methods directly
        registry.add_command("new_tab".to_string(), Box::new(Self::new_tab));
        registry.add_command("close_tab".to_string(), Box::new(Self::close_tab));
        registry.add_command("update_states".to_string(), Box::new(Self::update_states));
        registry.add_command(
            "update_tab_title".to_string(),
            Box::new(Self::update_tab_title),
        );

        registry.add_command("switch_tab".to_string(), Box::new(Self::switch_tab));
    }
}

#[tauri::command]
pub fn send_current_open_tab(id: String, state: State<'_, AppState>) {
    log::debug!("send_current_open_tab init");
    state.tab_switcher.lock().unwrap().current_tab_id = Some(id.clone());
}

#[tauri::command]
pub fn get_current_open_tab(state: State<'_, AppState>) -> Result<String, String> {
    log::debug!("get_current_open_tab init");
    return state
        .tab_switcher
        .lock()
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
        .lock()
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

    let title = check_path_exists(&trove_dir);

    // Clean up any stale entries in tabs and recent_files that don't exist on disk
    // but have the same title
    {
        state.tab_switcher.lock().unwrap().tabs.retain(|_, tab| {
            let file_path =
                trove_dir.join(sanitize_filename::sanitize(format!("{}.md", &tab.title)));
            file_path.exists() && tab.title != title
        });
    }
    {
        state.workspace.lock().unwrap().recent_files.retain(|file| {
            let file_path =
                trove_dir.join(sanitize_filename::sanitize(format!("{}.md", &file.title)));
            file_path.exists() && file.title != title
        });
    }

    // Create new tab
    let new_tab = Tab {
        id: new_id.clone(),
        title: title.clone(),
    };

    // Insert into IndexMap
    {
        let mut tab_switcher = state.tab_switcher.lock().unwrap();
        tab_switcher.tabs.insert(new_id.clone(), new_tab.clone());
        tab_switcher.current_tab_id = Some(new_id.clone());
    }

    {
        state
            .workspace
            .lock()
            .unwrap()
            .recent_files
            .push(RecentFileInfo {
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

fn check_path_exists(trove_dir: &Path) -> String {
    let mut iteration: u32 = 0;
    loop {
        let title = if iteration == 0 {
            sanitize_filename::sanitize("Untitled.md")
        } else {
            sanitize_filename::sanitize(format!("Untitled {}.md", &iteration))
        };

        let file_path = trove_dir.join(&title);
        if !file_path.exists() {
            return title.strip_suffix(".md").unwrap_or(&title).to_string();
        }
        iteration += 1;
    }
}

#[tauri::command]
pub fn update_tab_title(
    id: String,
    title: String,
    state: State<'_, AppState>,
) -> Result<Tab, String> {
    log::debug!("update_tab_title init");
    let tabs = &mut state.tab_switcher.lock().unwrap().tabs;

    // Check if the new title already exists in other tabs
    if tabs.values().any(|tab| tab.id != id && tab.title == title) {
        Err("A tab with this title already exists".to_string())
    } else {
        // Get the tab, update its title, and insert it back
        if let Some(mut tab) = tabs.get(&id).cloned() {
            tab.title = title;
            tabs.insert(id, tab.clone());
            Ok(tab)
        } else {
            Err("Tab not found".to_string())
        }
    }
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
        let mut tab_switcher = state.tab_switcher.lock().unwrap();
        tab_switcher.tabs.insert(id, new_tab.clone());
    }

    event_emitter(app);

    Ok(new_tab)
}

#[tauri::command]
pub fn close_tab(
    app: AppHandle,
    id: String,
    state: State<'_, AppState>,
) -> Result<Option<String>, String> {
    log::debug!("close_tab init");
    // let orig_state = &state;
    let tab_switcher = &mut state.tab_switcher.lock().unwrap();
    let tabs = &mut tab_switcher.tabs;

    if tabs.is_empty() {
        return Ok(None); // Don't close the last tab
    }

    if let Some((index, _, _)) = tabs.shift_remove_full(&id) {
        // Get the next tab ID (either at same index or last tab)
        let next_tab_id = tabs
            .get_index(index)
            .or_else(|| tabs.last())
            .map(|(id, _)| id.clone());

        // Update current open tab if needed
        if let Some(next_id) = &next_tab_id {
            tab_switcher.current_tab_id = Some(next_id.clone());
        }
        event_emitter(app);

        Ok(next_tab_id)
    } else {
        Err("Tab not found".to_string())
    }
}
