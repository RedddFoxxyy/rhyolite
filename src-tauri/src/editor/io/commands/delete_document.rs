use crate::{
	app_state::{AppState, AppStateInner, MarkdownFileData, TROVE_DIR, Tab},
	editor::{
		io::{
			IOCommands, commands::get_document_content::get_document_content_helper, get_trove_dir,
			save_user_data,
		},
		markdown_handler,
		tabs::update_tabs_state,
	},
};
use std::fs;
use tauri::{AppHandle, Emitter, Manager, State};

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
		let temp_app = app.clone();
		let state = &temp_app.state::<AppState>();
		let orig_state = &state;

		let Some(payload) = payload else {
			log::warn!("Invalid call to save_document");
			return;
		};

		if let Err(e) = serde_json::from_str::<String>(&payload) {
			log::error!("Invalid payload found, ERROR: {}", e);
			return;
		}

		let delete_tab_id = serde_json::from_str::<String>(&payload).unwrap();

		let next_tab_data = delete_document_helper(state, delete_tab_id).await;

		update_tabs_state(app.clone()).await;
		let _ = save_user_data(orig_state).await;

		if next_tab_data.is_none() {
			log::warn!("Failed to load document data!");
			return;
		}
		let html_output = markdown_handler::markdown_to_html(&next_tab_data.unwrap().content);

		let _ = app.emit("current_editor_content", html_output);
	}
}

// TODO: This function can be named better!
async fn delete_document_helper(
	state: &State<'_, AppStateInner>,
	delete_tab_id: String,
) -> Option<MarkdownFileData> {
	let (next_tab, delete_tab_title): (Tab, String) = {
		let mut tab_switcher = state.tab_switcher.write().await;

		let delete_tab_title = tab_switcher
			.tabs
			.get(&delete_tab_id)
			.map(|tab| tab.title.clone())
			.unwrap_or_else(|| panic!("Tab title does not exist"));

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
				log::info!("Cannot delete the only open tab(this will be fixed in future).");
				return None;
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

	// Remove the file in current_tab from the Recent Files in
	// a seperate scope to avoid deadlock.
	{
		let mut workspace = state.workspace.write().await;

		workspace.recent_files.retain(|doc| doc.id != delete_tab_id);
	}

	// Handle file operations
	let trove_dir = get_trove_dir(TROVE_DIR);
	let filename = sanitize_filename::sanitize(format!("{}.md", delete_tab_title));
	let file_path = trove_dir.join(&filename);

	if file_path.exists() {
		let _ = fs::remove_file(&file_path)
			.map_err(|e| format!("Failed to delete file {}: {}", file_path.display(), e));
	}

	// Get the DocumentData for the next tab
	{
		// send_document_content(next_tab, app);
		//TODO: Handle panic cases here when using unwrap.
		get_document_content_helper(Tab {
			id: next_tab.id,
			title: next_tab.title,
		})
	}
}
