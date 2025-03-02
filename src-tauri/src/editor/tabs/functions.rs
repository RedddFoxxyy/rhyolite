//TODO: We can name this file something better instead of naming it as functions.

use tauri::{AppHandle, Emitter, Manager, State};
use uuid::Uuid;

//use crate::commands::event_emitter;

use crate::{
    app_state::{AppState, CommandRegistrar, CommandRegistry, DocumentData, Tab},
    editor::io::{commands::save_document::save_document_helper, get_trove_dir, save_user_data},
    utils::generate_available_path,
    FileInfo,
};
// use std::sync::{Arc, Mutex};

pub struct TabCommands;

impl TabCommands {
    pub fn update_states(app: AppHandle, _payload: Option<String>) {
        update_tabs_state(app);
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
        registry.add_command("load_tab".to_string(), Box::new(Self::load_tab));
        registry.add_command("cycle_tabs".to_string(), Box::new(Self::cycle_tabs));
        registry.add_command("goto_tab_1".to_string(), Box::new(Self::goto_tab_1));
        registry.add_command("goto_last_tab".to_string(), Box::new(Self::goto_last_tab));
    }
}

pub fn cleanup_deleted_files_workaround(
    state: &State<'_, crate::app_state::AppStateInner>,
    trove_dir: std::path::PathBuf,
    title: &String,
) {
    // Clean up any stale entries in tabs and recent_files that don't exist on disk
    // but have the same title

    let maybe_tab_switcher = state.get_tab_switcher_mut();
    let maybe_workspace = state.get_workspace_mut();

    if maybe_tab_switcher.is_none() || maybe_workspace.is_none() {
        log::error!("Failed to clean garbage files.");
        return;
    }

    let mut tab_switcher = maybe_tab_switcher.unwrap();
    let mut workspace = maybe_workspace.unwrap();

    tab_switcher.tabs.retain(|_, tab| {
        let file_path = trove_dir.join(sanitize_filename::sanitize(format!("{}.md", &tab.title)));
        file_path.exists() && tab.title != *title
    });

    workspace.recent_files.retain(|file| {
        let file_path = trove_dir.join(sanitize_filename::sanitize(format!("{}.md", &file.title)));
        file_path.exists() && file.title != *title
    });
}

#[tauri::command]
pub fn send_current_open_tab(id: String, state: State<'_, AppState>) {
    log::debug!("send_current_open_tab init");
    state.tab_switcher.write().unwrap().current_tab_id = Some(id.clone());
}

#[tauri::command]
pub fn get_current_open_tab(state: State<'_, AppState>) -> Result<String, String> {
    log::debug!("get_current_open_tab init");
    state
        .tab_switcher
        .read()
        .unwrap()
        .current_tab_id
        .clone()
        .ok_or("Tab doesn't exist".to_string())
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
    update_tabs_state(app);
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
    save_document_helper(
        orig_state,
        DocumentData {
            id: new_id,
            title,
            content: String::new(),
        },
    );
    update_tabs_state(app);

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

    update_tabs_state(app);

    Ok(new_tab)
}

// I named this function based on the fact that it updates the reactive states
// related to tabs on the frontend when called.
// OPTIMIZE: 1. Use exec_command with new_tab command call
// instead of using the new_tab function defined outside
// the TabCommands Struct.
//
// 2. Improve Error Handling.
pub fn update_tabs_state(app: AppHandle) {
    let state = app.state::<AppState>();

    // Get current tab ID, create new tab if none exists
    let current_tab_id = {
        let maybe_tab_switcher = state.get_tab_switcher();
        if maybe_tab_switcher.is_none() {
            log::error!("Failed to update UI to new state!!!");
            return;
        }
        let tab_switcher = maybe_tab_switcher.unwrap();
        match &tab_switcher.current_tab_id {
            Some(id) => id.clone(),
            None => {
                // Release the lock before creating a new tab
                drop(tab_switcher);
                match new_tab(app.clone()) {
                    Ok(new_tab) => new_tab.id,
                    Err(e) => {
                        log::error!("Failed to create new tab: {}", e);
                        return; // Exit the function if we can't create a new tab
                    }
                }
            }
        }
    };

    // Emit all the tabs
    {
        let maybe_tab_switcher = state.get_tab_switcher();

        if maybe_tab_switcher.is_none() {
            log::error!("Failed to update UI to new state!!!");
            return;
        }

        let tab_switcher = maybe_tab_switcher.unwrap();

        let tabs: Vec<Tab> = tab_switcher.tabs.values().cloned().collect();
        let _ = app.emit("Tabs", tabs);
    }

    // Emit current tab
    {
        let maybe_tab_switcher = state.get_tab_switcher();

        if maybe_tab_switcher.is_none() {
            log::error!("Failed to update UI to new state!!!");
            return;
        }

        let tab_switcher = maybe_tab_switcher.unwrap();

        if let Some(current_tab) = tab_switcher.tabs.get(&current_tab_id) {
            let _ = app.emit("Current_Tab", current_tab.clone());
        }
    }
}
