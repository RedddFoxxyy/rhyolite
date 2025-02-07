use crate::app_state::{AppState, CommandRegistrar};
//use crate::editor::tabs;
use crate::editor::{tabs::TabCommands, io::IOCommands};

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
            .map(|p| format!("\"{}\"", p.escape_default()))
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
    IOCommands::register_commands(&mut command_registry);
}
