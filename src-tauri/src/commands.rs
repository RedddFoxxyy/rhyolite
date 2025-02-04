use crate::app_state::{AppState, CommandRegistrar, Tab};
use crate::editor::tabs;
use crate::editor::tabs::TabCommands;
use tauri::{AppHandle, Emitter, Manager};

/// TODO: The current organisation of exec_command function is not good
/// We might need to change this and make it better and scalable.
///
/// 1. Improve handling of incoming payload(json).
/// 2. Add more error handling so that app does not panic!
#[tauri::command]
pub fn exec_command(cmd: String, payload: Option<String>, app: AppHandle) {
    log::debug!(
        "command::exec: {}({})",
        cmd,
        payload
            .clone()
            .map(|p| format!("\"{}\"", p.escape_default().to_string()))
            .unwrap_or("".to_string())
    );

    let state = app.state::<AppState>();

    if let Some(command_item) = state
        .command_registry
        .lock()
        .unwrap()
        .commands
        .get_mut(&cmd)
    {
        let action = &mut command_item.action;
        (action)(app.clone(), payload);
    } else {
        log::debug!("Unknown command: {}", cmd);
        let _ = app.emit("dummy-event", "hellllo");
    };
}

pub fn load_default_commands(app: &AppHandle) {
    let app_state = app.state::<AppState>();
    let mut command_registry = app_state
        .command_registry
        .lock()
        .expect("Failed accessing command registry");

    // Register commands from each module
    TabCommands::register_commands(&mut command_registry);
}

pub fn event_emitter(app: AppHandle) {
    let state = app.state::<AppState>();
    let current_state = state;

    // Get current tab ID, create new tab if none exists
    let current_tab_id = {
        let tab_switcher = current_state.tab_switcher.read().unwrap();
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
            .read()
            .unwrap()
            .tabs
            .values()
            .cloned()
            .collect();
        let _ = app.emit("Tabs", tabs);
    }

    // Emit current tab
    {
        let tab_switcher = current_state.tab_switcher.read().unwrap();
        if let Some(current_tab) = tab_switcher.tabs.get(&current_tab_id) {
            let _ = app.emit("Current_Tab", current_tab.clone());
        }
    }
}
