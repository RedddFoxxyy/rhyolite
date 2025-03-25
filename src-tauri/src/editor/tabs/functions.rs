//TODO: We can name this file something better instead of naming it as functions.

use tauri::{AppHandle, Emitter, Manager, State};

//use crate::commands::event_emitter;

use crate::app_state::{AppState, CommandRegistrar, CommandRegistry, Tab};
// use std::sync::{Arc, Mutex};

pub struct TabCommands;

impl TabCommands {
	pub async fn update_states(app: AppHandle, _payload: Option<String>) {
		update_tabs_state(app).await;
	}
}

impl CommandRegistrar for TabCommands {
	fn register_commands(registry: &mut CommandRegistry) {
		// Register the methods directly
		registry.add_command(
			"new_tab".to_string(),
			Box::new(|app, payload| Box::pin(Self::new_tab(app, payload))),
		);
		registry.add_command(
			"close_tab".to_string(),
			Box::new(|app, payload| Box::pin(Self::close_tab(app, payload))),
		);
		registry.add_command(
			"update_states".to_string(),
			Box::new(|app, payload| Box::pin(Self::update_states(app, payload))),
		);
		registry.add_command(
			"update_tab_title".to_string(),
			Box::new(|app, payload| Box::pin(Self::update_tab_title(app, payload))),
		);
		registry.add_command(
			"switch_tab".to_string(),
			Box::new(|app, payload| Box::pin(Self::switch_tab(app, payload))),
		);
		registry.add_command(
			"load_tab".to_string(),
			Box::new(|app, payload| Box::pin(Self::load_tab(app, payload))),
		);
		registry.add_command(
			"cycle_tabs".to_string(),
			Box::new(|app, payload| Box::pin(Self::cycle_tabs(app, payload))),
		);
		registry.add_command(
			"goto_tab_1".to_string(),
			Box::new(|app, payload| Box::pin(Self::goto_tab_1(app, payload))),
		);
		registry.add_command(
			"goto_last_tab".to_string(),
			Box::new(|app, payload| Box::pin(Self::goto_last_tab(app, payload))),
		);
	}
}

pub async fn cleanup_deleted_files_workaround(
	state: &State<'_, crate::app_state::AppStateInner>,
	trove_dir: std::path::PathBuf,
	title: &String,
) {
	// Clean up any stale entries in tabs and recent_files that don't exist on disk
	// but have the same title

	let mut tab_switcher = state.tab_switcher.write().await;
	let mut workspace = state.workspace.write().await;

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
pub async fn get_current_open_tab(state: State<'_, AppState>) -> Result<String, String> {
	log::debug!("get_current_open_tab init");
	let tab_switcher = state.tab_switcher.read().await;

	tab_switcher
		.current_tab_id
		.clone()
		.ok_or("Tab doesn't exist".to_string())
}

#[tauri::command]
pub async fn get_tabs(state: State<'_, AppState>) -> Result<Vec<Tab>, String> {
	log::debug!("get_tabs init");
	let tab_switcher = state.tab_switcher.read().await;

	Ok(tab_switcher.tabs.values().cloned().collect())
}

#[tauri::command]
pub async fn update_states(app: AppHandle) {
	update_tabs_state(app).await;
}

#[tauri::command]
pub async fn load_tab(
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
		let mut tab_switcher = state.tab_switcher.write().await;
		tab_switcher.tabs.insert(id, new_tab.clone());
	}

	update_tabs_state(app).await;

	Ok(new_tab)
}

// I named this function based on the fact that it updates the reactive states
// related to tabs on the frontend when called.
// OPTIMIZE: 1. Use exec_command with new_tab command call
// instead of using the new_tab function defined outside
// the TabCommands Struct.
//
// 2. Improve Error Handling.
pub async fn update_tabs_state(app: AppHandle) {
	let state = app.state::<AppState>();

	// Get current tab ID, create new tab if none exists
	// TODO: Handle case where current tab id will be none
	let current_tab_id: String = {
		let tab_switcher = state.tab_switcher.read().await;

		if tab_switcher.current_tab_id.is_none() {
			log::error!("Failed to update UI to new state!!!");
			return;
		}
		tab_switcher.current_tab_id.clone().unwrap()
	};

	{
		// Emit all the tabs
		let tab_switcher = state.tab_switcher.read().await;

		let tabs: Vec<Tab> = tab_switcher.tabs.values().cloned().collect();
		let _ = app.emit("Tabs", tabs);

		// Emit current tab
		if let Some(current_tab) = tab_switcher.tabs.get(&current_tab_id) {
			let _ = app.emit("Current_Tab", current_tab.clone());
		}
	}
}
