#![allow(unused_imports)]

use crate::{
    app_state::{AppState, CommandRegistrar},
    editor::{io::IOCommands, settings::themes::ThemeCommands, tabs::TabCommands},
};

use tauri::{AppHandle, Emitter, Manager};
use tokio;
use tokio::task::spawn_blocking;

/// Executes a command by name with an optional payload, running the associated action asynchronously.
///
/// This function is a Tauri command handler that takes a command name (`cmd`), an optional `payload`,
/// and the Tauri `AppHandle`. It retrieves the command's action from the app's state, executes it in
/// a separate async task, and waits for completion. The design ensures thread-safety and avoids
/// capturing non-`Send` types (like raw pointers) across await points, making it compatible with
/// Tauri's multi-threaded runtime.
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
// 3. Use an async mutex (e.g., tokio::sync::Mutex) instead of parking_lot::Mutex
#[tauri::command]
pub async fn exec_command(cmd: String, payload: Option<String>, app: AppHandle) {
    // Log the command being executed for debugging purposes
    log::debug!("command::exec: {}", cmd);

    // Get the app's state, which holds the command registry and other data
    let state = app.state::<AppState>();

    // Scope the mutex lock to fetch the command's future without holding the lock across an await
    let future_opt = {
        // Try to acquire the command registry mutex; this is synchronous but short-lived
        let mut command_registry = match state.get_command_registry() {
            Some(registry) => registry,
            None => {
                log::error!("Failed to execute the command {}", cmd);
                return;
            }
        };

        // Look up the command in the registry and map it to its action's future
        command_registry
            .commands
            .get_mut(&cmd) // Get a mutable reference to the command item, if it exists
            .map(|command_item| {
                let action = &mut command_item.action; // Borrow the action closure mutably
                // Call the action with cloned app handle and payload, returning a pinned future
                (action)(app.clone(), payload)
            })
        // Scope ends here, dropping the mutex guard before any async operations
    };

    // Now outside the mutex scope, handle the future if we got one
    if let Some(future) = future_opt {
        // Spawn the future as a separate async task to isolate its execution
        // tokio::spawn takes a Send future and runs it concurrently on the runtime
        let handle = tokio::spawn(future);

        // Wait for the spawned task to complete; this is async and non-blocking
        handle.await.unwrap();
    } else {
        // No command found in the registry, log it and exit
        log::debug!("Unknown command: {}", cmd);
    }
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
    ThemeCommands::register_commands(&mut command_registry);
}
