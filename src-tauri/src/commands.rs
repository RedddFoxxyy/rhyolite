#![allow(unused_imports)]

use crate::{
    app_state::{AppState, CommandRegistrar},
    editor::{io::IOCommands, tabs::TabCommands},
};

use tauri::{AppHandle, Emitter, Manager};

/// # Execute Command
/// Executes the command with the given name(cmd) and passes the
/// given payload to the command while executing.
///
/// This is used by tauri events on the frontend(svelte) to execute
/// any io/tabs related logic on user input!
///
/// ___Example:___
///
/// On frontend you can invoke a command like this:
/// `invoke("exec_command", { cmd: "delete_document" });`
/// In the above given example we invoke and pass the tauri command `exec_command`,
/// and along with it pass the name of the command we want to execute `delete_document`.
///
/// > NOTE: Tauri command and command called by this function are two different things!
///
/// or you can ivoke a command like this, if u need to pass some data:
/// `invoke("exec_command", { cmd: "save_document", payload: JSON.stringify({ id: documentId, title: documentTitle, content: documentContent || ""})});`
/// In the above example, we are also passing a payload( which is optional ), and the payload is basically the document
/// data type( id, title, and document content).
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
