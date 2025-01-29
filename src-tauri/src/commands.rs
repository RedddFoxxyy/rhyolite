use crate::app_state::{AppState, CommandItem, Tab};
use crate::editor::{io, tabs};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager, State};

#[tauri::command]
pub fn exec_command(cmd: String, payload: serde_json::Value, app: AppHandle) {
    log::debug!("exec command '{}' called with '{}'", cmd, payload);

    let state = app.state::<AppState>();
    let mut state_lock = state.lock().unwrap();

    if let Some(command_item) = state_lock.command_registry.commands.get_mut(&cmd) {
        (command_item.action)(app.clone(), payload.to_string());
    } else {
        log::debug!("Unknown command: {}", cmd);
        if payload.is_object() {
            let value = payload.as_object().unwrap().get("hi");
            log::debug!("value of hi: '{:?}'", value);
        }
        let _ = app.emit("dummy-event", "hellllo");
    }
}

pub fn add_commands_to_registry(app: AppHandle) {
    // let state = state.clone();
    let action = Box::new(move |app: AppHandle, _: String| {
        if let Ok(tab) = tabs::new_tab(app) {
            log::debug!("created new tab");
        }
    });

    let app_state = app.state::<AppState>();

    if let Ok(mut command_registry) = app_state.command_registry.lock() {
        command_registry.add_command("new_tab".to_string(), action);
    }
    //command_registry.add_command("new_tab".to_string(), action);
}

pub fn event_emitter(app: AppHandle) {
    let state = app.state::<AppState>();
    let current_state = state;
    let current_tab_id;
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
    {
        current_tab_id = current_state
            .tab_switcher
            .lock()
            .unwrap()
            .current_tab_id
            .clone()
            .unwrap();
    }
    {
        let current_tab = current_state
            .tab_switcher
            .lock()
            .unwrap()
            .tabs
            .get(&current_tab_id)
            .map(|tab| (*tab).clone())
            .unwrap();
        let _ = app.emit("Current_Tab", current_tab);
    }
}
