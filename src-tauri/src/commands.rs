use crate::app_state::{AppState, CommandItem, Tab};
use crate::editor::{io, tabs};
// use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager, State};

#[tauri::command]
pub fn exec_command(cmd: String, payload: serde_json::Value, app: AppHandle) {
    log::debug!("exec command '{}' called with '{}'", cmd, payload);

    let state = app.state::<AppState>();

    if let Some(command_item) = state.command_registry.lock().unwrap().commands.get_mut(&cmd) {
        let mut action = command_item.action.lock().unwrap();
        (action)(app.clone(), payload.to_string());
    } else {
        log::debug!("Unknown command: {}", cmd);
        if payload.is_object() {
            let value = payload.as_object().unwrap().get("hi");
            log::debug!("value of hi: '{:?}'", value);
        }
        let _ = app.emit("dummy-event", "hellllo");
    };
}

pub fn add_commands_to_registry(app: AppHandle) {
    // let state = state.clone();
    // let app_state_temp = app.clone();
    let action = Box::new(move |app: AppHandle, _: String| {
        if let Ok(tab) = tabs::new_tab(app.clone()) {
            log::debug!("created new tab");
        }
    });

    let app_state = app.state::<AppState>();

    if let Ok(mut command_registry) = app_state.command_registry.lock() {
        command_registry.add_command("new_tab".to_string(), action);
    };
    //command_registry.add_command("new_tab".to_string(), action);
}

pub fn event_emitter(app: AppHandle) {
    let state = app.state::<AppState>();
    let current_state = state;


    // Get current tab ID, create new tab if none exists
    let current_tab_id = {
        let tab_switcher = current_state.tab_switcher.lock().unwrap();
        match &tab_switcher.current_tab_id {
            Some(id) => id.clone(),
            None => {
                // Release the lock before creating a new tab
                drop(tab_switcher);
                match tabs::new_tab(app.clone()) {
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
        let tabs: Vec<Tab> = current_state
            .tab_switcher
            .lock()
            .unwrap()
            .tabs
            .values()
            .cloned()
            .collect();
        let _ = app.emit("Tabs", tabs);
    }

    // Emit current tab
    {
        let tab_switcher = current_state.tab_switcher.lock().unwrap();
        if let Some(current_tab) = tab_switcher.tabs.get(&current_tab_id) {
            let _ = app.emit("Current_Tab", current_tab.clone());
        }
    }
}
