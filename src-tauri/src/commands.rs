#![allow(unused_imports)]

use crate::{
	app_state::{AppState, CommandRegistrar},
	editor::{io::IOCommands, settings::themes::ThemeCommands, tabs::TabCommands},
};

use tauri::{AppHandle, Emitter, Manager};

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
	let state = app.state::<AppState>();
	let mut command_registry = state.command_registry.lock().await;

	// Retrieve the action and release the lock
	let future = {
		if let Some(command_item) = command_registry.commands.get_mut(&cmd) {
			let action = &mut command_item.action;
			Some((action)(app.clone(), payload)) // Get the future
		} else {
			None
		}
	};

	if let Some(future) = future {
		future.await;
	} else {
		log::error!("Failed to execute the command {}", cmd);
	}
}

pub async fn load_default_commands(app: &AppHandle) {
	let app_state = app.state::<AppState>();

	let mut command_registry = app_state.command_registry.lock().await;

	// Register commands from each module
	TabCommands::register_commands(&mut command_registry);
	IOCommands::register_commands(&mut command_registry);
	ThemeCommands::register_commands(&mut command_registry);
}
