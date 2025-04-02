//TODO: We can name this file something better instead of naming it as functions.

use std::fs;

use tauri::{AppHandle, Emitter, Manager, State};
use uuid::Uuid;

//use crate::commands::event_emitter;

use crate::{
	app_state::{
		AppState, CommandRegistrar, CommandRegistry, DEFAULT_NOTE_TITLE, MarkdownFileData,
		TROVE_DIR, Tab,
	},
	editor::io::{
		generate_available_path, get_trove_dir, save_document_helper, save_user_data,
		send_document_content,
	},
};
// use std::sync::{Arc, Mutex};

pub struct TabCommands;

impl TabCommands {
	pub async fn update_states(app: AppHandle, _payload: Option<String>) {
		update_tabs_state(app).await;
	}

	pub async fn new_tab(app: AppHandle, _payload: Option<String>) {
		log::debug!("new_tab init");

		let temp_app = app.clone();
		let state = &temp_app.state::<AppState>();

		let new_id = Uuid::new_v4().to_string();

		let trove_dir = get_trove_dir(TROVE_DIR);

		let new_path =
			generate_available_path(trove_dir.join(String::from(DEFAULT_NOTE_TITLE) + ".md"));
		let title = new_path.file_stem().unwrap().to_string_lossy().to_string();

		cleanup_deleted_files_workaround(state, trove_dir, &title).await;

		// Create new tab
		let new_tab = Tab {
			id: new_id.clone(),
			title: title.clone(),
		};

		// Insert into IndexMap
		let mut tab_switcher = state.tab_switcher.write().await;
		tab_switcher.tabs.insert(new_id.clone(), new_tab.clone());
		tab_switcher.current_tab_id = Some(new_id.clone());

		drop(tab_switcher); // drop the write lock to avoid deadlock

		let save_user_data_error = save_user_data(state).await;
		if let Err(error) = save_user_data_error {
			log::error!("Failed to save user data: {}", error);
		}

		let save_document_data = MarkdownFileData {
			id: new_id,
			title,
			content: String::new(),
		};
		save_document_helper(state, save_document_data).await;
		update_tabs_state(app.clone()).await;
		send_document_content(Some(new_tab), app).await;
	}

	pub async fn close_tab(app: AppHandle, payload: Option<String>) {
		log::debug!("close_tab init");
		let Some(payload) = payload else {
			log::warn!("Invalid call to close_tab");
			return;
		};

		// Parse the JSON payload
		let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&payload) else {
			log::debug!("Failed to parse JSON payload");
			return;
		};

		let Some(tab_id) = json_value.get("tabId").and_then(|v| v.as_str()) else {
			log::debug!("Invalid JSON payload format: missing or invalid tabId field");
			return;
		};

		let temp_app = app.clone();
		let state = temp_app.state::<AppState>();

		let mut tab_switcher = state.tab_switcher.write().await;
		let mut workspace = state.workspace.write().await;
		let tabs = &tab_switcher.tabs;

		// Do not close the only remaining tab. This will be removed in future..
		if tabs.len() == 1 {
			return;
		}

		let next_tab_index = tab_switcher.tabs.shift_remove_full(tab_id).unwrap().0;

		let next_tab: Tab;

		if let Some(next_tab_kv) = tab_switcher.tabs.get_index(next_tab_index) {
			next_tab = next_tab_kv.1.clone();
		} else {
			next_tab = tab_switcher
				.tabs
				.get_index(next_tab_index - 1)
				.unwrap()
				.1
				.clone();
		}

		tab_switcher.current_tab_id = Some(next_tab.id.clone());

		// Remove the file in closing_tab from the documents cache.
		workspace.documents.retain(|id, _| id != tab_id);

		drop(tab_switcher); // drop the write lock to avoid deadlock
		drop(workspace); // drop the write lock to avoid deadlock

		// Call event emitter after releasing the lock
		update_tabs_state(app.clone()).await;
		send_document_content(Some(next_tab), app).await;
	}

	pub async fn switch_tab(app: AppHandle, payload: Option<String>) {
		let Some(payload) = payload else {
			log::warn!("Invalid call to switch_tab");
			return;
		};
		let temp_app = app.clone();
		let state = temp_app.state::<AppState>();

		let _tab_switch_lock = state.active_tab_switch.lock().await;
		if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&payload) {
			if let Some(tab_id) = json_value.get("tabId").and_then(|v| v.as_str()) {
				let mut tab_switcher = state.tab_switcher.write().await;

				if tab_switcher.tabs.values().any(|tab| tab.id == tab_id) {
					tab_switcher.current_tab_id = Some(tab_id.to_string());
				}

				let current_tab_data = tab_switcher.tabs.get(tab_id).cloned();
				send_document_content(current_tab_data, app.clone()).await;
			}
		}
		update_tabs_state(app).await;
	}

	pub async fn update_tab_title(app: AppHandle, payload: Option<String>) {
		log::debug!("update_tab_title init");
		let Some(payload) = payload else {
			log::warn!("Invalid call to update_tab_title");
			return;
		};

		if let Ok(payload_object) = serde_json::from_str::<Tab>(&payload) {
			let temp_app = app.clone();
			let state = temp_app.state::<AppState>();

			let mut tab_switcher = state.tab_switcher.write().await;
			let tabs = &mut tab_switcher.tabs;
			let new_title = payload_object.title;
			let id = payload_object.id;

			let mut old_title = None;

			// Check if the new title already exists in other tabs
			if tabs
				.values()
				.any(|tab| tab.id != id && tab.title == new_title)
			{
				log::debug!("A tab with this title already exists.");
			} else {
				// Get the tab, update its title, and insert it back
				if let Some(mut tab) = tabs.get(&id).cloned() {
					old_title = Some(tab.title.clone());
					tab.title = new_title;
					tabs.insert(id, tab.clone());
				} else {
					log::debug!("Tab not found");
				}
			}

			// Delete the file with old title, if old title exists.
			if old_title.is_some() {
				let trove_dir = get_trove_dir(TROVE_DIR);
				let filename = sanitize_filename::sanitize(format!("{}.md", old_title.unwrap()));
				let file_path = trove_dir.join(&filename);

				if file_path.exists() {
					let _ = fs::remove_file(&file_path).map_err(|e| {
						format!("Failed to delete file {}: {}", file_path.display(), e)
					});
				}
			}
		}

		update_tabs_state(app).await;
	}

	pub async fn load_tab(app: AppHandle, payload: Option<String>) {
		log::debug!("load_tab init");
		let Some(payload) = payload else {
			log::warn!("Invalid call to load_tab");
			return;
		};
		if let Ok(tab_data) = serde_json::from_str::<Tab>(&payload) {
			let temp_app = app.clone();
			let state = temp_app.state::<AppState>();

			let mut tab_switcher = state.tab_switcher.write().await;
			tab_switcher
				.tabs
				.insert(tab_data.id.clone(), tab_data.clone());

			tab_switcher.current_tab_id = Some(tab_data.id.clone());

			drop(tab_switcher); // Drop the lock to avoid deadlock

			update_tabs_state(app.clone()).await;
			send_document_content(Some(tab_data), app).await;
		}
	}

	pub async fn cycle_tabs(app: AppHandle, _payload: Option<String>) {
		log::debug!("Init Cycle Tabs");
		let temp_app = app.clone();
		let state = temp_app.state::<AppState>();

		let _tab_switch_lock = state.active_tab_switch.lock().await;
		let mut tab_switcher = state.tab_switcher.write().await;

		let current_tab_id = tab_switcher.current_tab_id.clone();

		if current_tab_id.is_none() {
			log::error!("No tab open!");
			return;
		}

		let current_tab_index = tab_switcher.tabs.get_index_of(&current_tab_id.unwrap());
		if current_tab_index.is_none() {
			log::error!("Invalid current tab, cannot cycle through tabs!!");
			return;
		}
		let next_tab_index = current_tab_index.unwrap() + 1;
		let next_tab_id: String = {
			let maybe_tab = tab_switcher.tabs.get_index(next_tab_index);
			if let Some(tab) = maybe_tab {
				tab.0.clone()
			} else {
				// TODO: Handle the last tab logic, that is no next tab.
				// It will cycle to the first tab in this case.
				tab_switcher.tabs.get_index(0).unwrap().0.clone()
				// return;
			}
		};

		if tab_switcher.tabs.values().any(|tab| tab.id == next_tab_id) {
			tab_switcher.current_tab_id = Some(next_tab_id.clone());
		}
		let current_tab_data = tab_switcher.tabs.get(&next_tab_id).cloned();

		drop(tab_switcher); // Drop tab_switcher to avoid deadlock

		update_tabs_state(app.clone()).await;
		send_document_content(current_tab_data, app).await;
	}

	pub async fn goto_tab_1(app: AppHandle, _payload: Option<String>) {
		log::debug!("Init goto_tab_1");
		let temp_app = app.clone();
		let state = temp_app.state::<AppState>();

		let _tab_switch_lock = state.active_tab_switch.lock().await;
		let mut tab_switcher = state.tab_switcher.write().await;

		if tab_switcher.tabs.is_empty() {
			log::info!("No tab open in the workspace.");
			return;
		}
		let first_tab_id: String = tab_switcher.tabs.get_index(0).unwrap().0.clone();

		if tab_switcher.tabs.values().any(|tab| tab.id == first_tab_id) {
			// Update current open tab if needed
			tab_switcher.current_tab_id = Some(first_tab_id.clone());
		}

		let current_tab_data = tab_switcher.tabs.get(&first_tab_id).cloned();

		drop(tab_switcher); // Drop tab_switcher to avoid deadlock

		update_tabs_state(app.clone()).await;
		send_document_content(current_tab_data, app).await;
	}

	pub async fn goto_last_tab(app: AppHandle, _payload: Option<String>) {
		log::debug!("Init goto_last_tab");
		let temp_app = app.clone();
		let state = temp_app.state::<AppState>();

		let _tab_switch_lock = state.active_tab_switch.lock().await;
		let mut tab_switcher = state.tab_switcher.write().await;

		if tab_switcher.tabs.is_empty() {
			log::info!("No tab open in the workspace.");
			return;
		}

		// NOTE: The getting of last tab may be optimised.
		let last_tab_entry = tab_switcher.tabs.last_entry();
		if last_tab_entry.is_none() {
			log::error!("Failed to get last tab entry!");
			return;
		}

		let last_tab_id = last_tab_entry.unwrap().get().id.clone();

		if tab_switcher.tabs.values().any(|tab| tab.id == last_tab_id) {
			// Update current open tab if needed
			tab_switcher.current_tab_id = Some(last_tab_id.clone());
		}

		let current_tab_data = tab_switcher.tabs.get(&last_tab_id).cloned();

		drop(tab_switcher); // Drop tab_switcher to avoid deadlock

		update_tabs_state(app.clone()).await;
		send_document_content(current_tab_data, app).await;
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
