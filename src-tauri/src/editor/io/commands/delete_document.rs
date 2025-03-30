use crate::{
	app_state::{AppState, Tab, TROVE_DIR},
	editor::{
		io::{
			get_trove_dir, IOCommands
		}, tabs::update_tabs_state}
	,
};
use std::fs;
use tauri::{AppHandle, Manager};

use super::get_document_content::send_document_content;

// use super::get_document_content::send_document_content;

impl IOCommands {
	/// # Delete Document!
	///
	/// Get's the current open tab and the next tab and deletes the file
	/// in the current tab and also removes it`s tab from app state.
	///
	/// The next tab is the tab after the current tab(if it exists) or
	/// the tab before it if the above condition is not true.
	///
	/// ___Example:(frontend)___
	/// ```ignore
	/// invoke("exec_command", { cmd: "delete_document" });
	/// ```
	/// TODO: Delete the tab that is passed as payload rather than deleting
	/// the current open tab.
	pub async fn delete_document(app: AppHandle, payload: Option<String>) {
		log::debug!("delete_document init");

		let Some(payload) = payload else {
			log::warn!("Invalid call to save_document");
			return;
		};

		if let Err(e) = serde_json::from_str::<String>(&payload) {
			log::error!("Invalid payload found, ERROR: {}", e);
			return;
		}

		let delete_tab_id = serde_json::from_str::<String>(&payload).unwrap();

		delete_document_helper(app.clone(), delete_tab_id).await;
	}
}

// TODO: This function can be named better!
async fn delete_document_helper(app: AppHandle, delete_tab_id: String) {
	let temp_app = app.clone();
	let state = &temp_app.state::<AppState>();

	let (next_tab, delete_tab_title): (Tab, String) = {
		let mut tab_switcher = state.tab_switcher.write().await;

		let delete_tab_title = tab_switcher
			.tabs
			.get(&delete_tab_id)
			.map(|tab| tab.title.clone())
			.unwrap_or_else(|| panic!("Tab title does not exist"));

		// Do not close the only remaining tab. This will be removed in future..
		if tab_switcher.tabs.len() == 1 {
			return;
		}
		
		let next_tab_index = tab_switcher
			.tabs
			.shift_remove_full(&delete_tab_id)
			.unwrap()
			.0;

		let next_tab: Tab;

		// TODO: Allow deletion of only remaining tab too, the editor should also be
		// able to handle no open tabs.
		if let Some(next_tab_kv) = tab_switcher.tabs.get_index(next_tab_index) {
			next_tab = next_tab_kv.1.clone();
		} else {
			if tab_switcher.tabs.is_empty() {
				log::info!("No Document open to be deleted.");
				return;
			}
			next_tab = tab_switcher
				.tabs
				.get_index(next_tab_index - 1)
				.unwrap()
				.1
				.clone();
		}

		tab_switcher.current_tab_id = Some(next_tab.id.clone());

		(next_tab, delete_tab_title)
	};

	// Remove the file in deleted_tab from the Recent Files and documents cache.
	let mut workspace = state.workspace.write().await;
	workspace.recent_files.retain(|doc| doc.id != delete_tab_id);
	workspace
		.documents
		.retain(|tabid, _| *tabid != delete_tab_id);
	
	drop(workspace);	// drop workspace to avoid deadlock.

	// Handle file operations
	let trove_dir = get_trove_dir(TROVE_DIR);
	let filename = sanitize_filename::sanitize(format!("{}.md", delete_tab_title));
	let file_path = trove_dir.join(&filename);

	if file_path.exists() {
		let _ = fs::remove_file(&file_path)
			.map_err(|e| format!("Failed to delete file {}: {}", file_path.display(), e));
	}

	// Get the DocumentData for the next tab
	let current_tab_data = Tab {
		id: next_tab.id,
		title: next_tab.title,
	};

	update_tabs_state(app.clone()).await;
	send_document_content(Some(current_tab_data), app.clone()).await;
}
