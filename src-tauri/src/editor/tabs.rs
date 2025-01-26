//! This module provides document tabs related functions for the app.
use tauri::State;
use uuid::Uuid;

use crate::app_state::{AppState, Tab};

use crate::RecentFileInfo;
use std::path::Path;

use super::io::{get_trove_dir, save_document, save_user_data};

#[tauri::command]
pub fn send_current_open_tab(id: String, state: State<'_, AppState>) {
    log::debug!("send_current_open_tab init");
    state
        .lock()
        .unwrap()
        .tab_switcher
        .get_mut()
        .unwrap()
        .current_tab_id = Some(id.clone());
}

#[tauri::command]
pub fn get_current_open_tab(state: State<'_, AppState>) -> Result<String, String> {
    log::debug!("get_current_open_tab init");
    return state
        .lock()
        .unwrap()
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
        .lock()
        .unwrap()
        .tab_switcher
        .lock()
        .unwrap()
        .tabs
        .values()
        .cloned()
        .collect())
}

#[tauri::command]
pub fn new_tab(state: State<'_, AppState>) -> Result<Tab, String> {
    log::debug!("new_tab init");
    let orig_state = &state;
    let mut state = state.lock().unwrap();

    let new_id = Uuid::new_v4().to_string();

    let trove_dir = get_trove_dir("Untitled_Trove");

    let title = check_path_exists(&trove_dir);

    // Clean up any stale entries in tabs and recent_files that don't exist on disk
    // but have the same title
    {
        state.tab_switcher.get_mut().unwrap().tabs.retain(|_, tab| {
            let file_path =
                trove_dir.join(sanitize_filename::sanitize(format!("{}.md", &tab.title)));
            file_path.exists() && tab.title != title
        });

        state.workspace.recent_files.retain(|file| {
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
    state
        .tab_switcher
        .get_mut()
        .unwrap()
        .tabs
        .insert(new_id.clone(), new_tab.clone());

    state.workspace.recent_files.push(RecentFileInfo {
        id: new_id.clone(),
        // FIXME: hardcoded name may have conflict
        title: "Untitled".to_string(),
    });

    state.tab_switcher.get_mut().unwrap().current_tab_id = Some(new_id.clone());
    std::mem::drop(state);
    save_user_data(orig_state)?;
    let _ = save_document(new_id, title, String::new(), orig_state.to_owned());

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
    let mut state = state.lock().unwrap();
    let tabs = &mut state.tab_switcher.get_mut().unwrap().tabs;

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
pub fn load_tab(id: String, title: String, state: State<'_, AppState>) -> Result<Tab, String> {
    log::debug!("load_tab init");
    let mut state = state.lock().unwrap();
    let tabs = &mut state.tab_switcher.get_mut().unwrap().tabs;

    let new_tab = Tab {
        id: id.clone(),
        title,
    };

    tabs.insert(id, new_tab.clone());

    Ok(new_tab)
}

#[tauri::command]
pub fn close_tab(id: String, state: State<'_, AppState>) -> Result<Option<String>, String> {
    log::debug!("close_tab init");
    let mut state = state.lock().unwrap();
    let tab_switcher = &mut state.tab_switcher.get_mut().unwrap();
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

        Ok(next_tab_id)
    } else {
        Err("Tab not found".to_string())
    }
}
