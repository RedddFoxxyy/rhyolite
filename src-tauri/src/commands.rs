#![allow(unused_imports)]

use crate::{
    app_state::{AppState, CommandRegistrar},
    editor::{io::IOCommands, tabs::TabCommands},
};

use tauri::{AppHandle, Emitter, Manager};

// TODO: The current organisation of exec_command function is not good
// We might need to change this and make it better and scalable.
//
// 1. Improve handling of incoming payload(json).
// 2. Add more error handling so that app does not panic!
#[tauri::command]
pub fn exec_command(cmd: String, payload: Option<String>, app: AppHandle) {
    log::debug!(
        "command::exec: {}({})",
        cmd,
        payload
            .clone()
            .map(|p| format!("\"{}\"", p.escape_default()))
            .unwrap_or("".to_string())
    );

    let state = app.state::<AppState>();

    let command_registry_option = state.get_command_registry();

    if command_registry_option.is_none() {
        log::error!("Failed to execute the command {}", cmd);
        return;
    }

    let mut command_registry = command_registry_option.unwrap();
    if let Some(command_item) = command_registry.commands.get_mut(&cmd) {
        let action = &mut command_item.action;
        (action)(app.clone(), payload);
    } else {
        log::debug!("Unknown command: {}", cmd);
        // let _ = app.emit("dummy-event", "hellllo");
    };
}

pub fn load_default_commands(app: &AppHandle) {
    let app_state = app.state::<AppState>();

    let command_registry_option = app_state.get_command_registry();
    if command_registry_option.is_none() {
        log::error!("Failed to load the default commands!");
        return;
    }

    let mut command_registry = command_registry_option.unwrap();

    // Register commands from each module
    TabCommands::register_commands(&mut command_registry);
    IOCommands::register_commands(&mut command_registry);
}
