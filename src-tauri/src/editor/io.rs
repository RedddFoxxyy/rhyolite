//TODO: We can name this file something better instead of naming it as functions.

use std::path::PathBuf;
use std::{fs, sync::Arc}; //Filesystem module
use tauri::{AppHandle, Emitter, Manager, State, Window};
// use tauri_plugin_dialog::DialogExt; //DialogExt trait to show dialog boxes
use crate::app_state::{AppStateInner, DEFAULT_NOTE_TITLE, FileInfo};
use crate::{
	app_state::{
		APP_DATA_DIR, AppState, CommandRegistrar, CommandRegistry, DocumentContent,
		MarkdownFileData, TROVE_DIR, Tab, USER_DATA_DIR, UserData,
	},
	editor::tabs::update_tabs_state,
};
use dirs;

pub struct IOCommands;
impl IOCommands {
	pub async fn init_frontend_state(app: AppHandle, _payload: Option<String>) {
		log::debug!("load_last_open_tabs init");
		let temp_app = app.clone();
		let state = &temp_app.state::<AppState>();

		let tab_switcher = state.tab_switcher.read().await;
		let current_tab_id = tab_switcher.current_tab_id.clone();
		if current_tab_id.is_none() {
			log::error!("Failed to get current tab id");
			return;
		}
		let current_tab_data = tab_switcher.tabs.get(&current_tab_id.unwrap()).cloned();
		update_tabs_state(app.clone()).await;
		send_document_content(current_tab_data, app).await;
	}

	pub async fn get_recent_files_metadata(app: AppHandle, _payload: Option<String>) {
		let temp_app = app.clone();
		let state = &temp_app.state::<AppState>();
		if let Err(e) = save_user_data(state).await {
			log::error!("Warning: Failed to save user data: {}", e);
		}
		let appdata_dir = get_documents_dir().join(USER_DATA_DIR);
		let userdata_path = appdata_dir.join("userdata.json");

		let metadata = if userdata_path.exists() {
			let content_result = fs::read_to_string(&userdata_path);
			if let Err(e) = &content_result {
				log::error!(
					"Failed to get the metadata content from the userdata path: {}",
					e
				);
			}
			let content = content_result.unwrap();

			// Try to deserialize the JSON.
			let user_data_result = serde_json::from_str::<UserData>(&content);
			if let Err(e) = &user_data_result {
				log::error!(
					"Failed to get the user_data from the metadata content: {}",
					e
				);
			}

			user_data_result.unwrap().recent_files
		} else {
			log::error!("Failed to get recent files metadata, userdata_path does not exist!");
			Vec::new()
		};

		if let Err(emit_err) = app.emit("recent_files_metadata", metadata) {
			log::error!("Failed to send the recent files metadata: {:#?}", emit_err);
		}
	}

	//TODO: Cleanup unused variables.
	pub async fn get_document_content(app: AppHandle, payload: Option<String>) {
		let Some(payload) = payload else {
			log::warn!("Invalid call to get_document_content");
			return;
		};

		if let Ok(tab_data) = serde_json::from_str::<Tab>(&payload) {
			send_document_content(Some(tab_data), app.clone()).await;
		}
	}

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

	pub async fn save_document(app: AppHandle, payload: Option<String>) {
		let Some(payload) = payload else {
			log::warn!("Invalid call to save_document");
			return;
		};
		log::debug!("save_document init");
		let temp_app = app.clone();
		let state = &temp_app.state::<AppState>();

		if let Ok(document_data) = serde_json::from_str::<MarkdownFileData>(&payload) {
			save_document_helper(state, document_data).await;
		}
	}
}

impl CommandRegistrar for IOCommands {
	fn register_commands(registry: &mut CommandRegistry) {
		registry.add_command(
			"save_document".to_string(),
			Box::new(|app, payload| Box::pin(Self::save_document(app, payload))),
		);
		registry.add_command(
			"delete_document".to_string(),
			Box::new(|app, payload| Box::pin(Self::delete_document(app, payload))),
		);
		registry.add_command(
			"get_document_content".to_string(),
			Box::new(|app, payload| Box::pin(Self::get_document_content(app, payload))),
		);
		registry.add_command(
			"init_frontend_state".to_string(),
			Box::new(|app, payload| Box::pin(Self::init_frontend_state(app, payload))),
		);
		registry.add_command(
			"get_recent_files_metadata".to_string(),
			Box::new(|app, payload| Box::pin(Self::get_recent_files_metadata(app, payload))),
		);
	}
}

/// This function returns the path to the documents' directory.
pub fn get_documents_dir() -> PathBuf {
	#[cfg(target_os = "android")]
	{
		// On Android, use the app's private storage directory
		let path = PathBuf::from("/data/user/0/com.rhyolite.dev/Rhyolite");
		// Create the directory if it doesn't exist
		fs::create_dir_all(&path).expect("Could not create Rhyolite directory");
		path
	}

	#[cfg(not(target_os = "android"))]
	{
		// Original desktop behavior
		let mut path = dirs::document_dir().expect("Could not find Documents directory");
		// TODO: Use a const for this name.
		path.push(APP_DATA_DIR);
		// Create the directory if it doesn't exist
		fs::create_dir_all(&path).expect("Could not create Rhyolite directory");
		path
	}
}

/// This function returns the path to the default trove directory.
pub fn get_trove_dir(trove_name: &str) -> PathBuf {
	//Get the path to documents/Rhyolite.
	let documents_dir = get_documents_dir();

	//Append the default trove name to the 'documents/Rhyolite path'.
	let trove_dir = documents_dir.join(trove_name);

	//Then create the path 'documents/Rhyolite/trove_name' if it does not
	fs::create_dir_all(&trove_dir).expect("Could not create Trove directory");

	//retrun the path of the default trove directory.
	trove_dir
}

/// Runs when the app is closing and saves the user data.
pub async fn on_app_close(window: &Window) {
	log::debug!("on_app_close init");
	let state = window.state::<AppState>();

	if let Err(err_saving) = save_user_data(&state).await {
		log::error!(
			"Failed to save the workspace before closing: {}",
			err_saving
		);
	}
}

/// This function saves the user data to the userdata.json file.
pub async fn save_user_data(state: &State<'_, AppState>) -> Result<(), String> {
	let user_data = {
		let tab_switcher = state.tab_switcher.read().await;
		let workspace = state.workspace.write().await;

		UserData {
			active_tabs: tab_switcher.tabs.values().cloned().collect(),
			last_open_tab: tab_switcher.current_tab_id.clone().unwrap(),
			recent_files: workspace.recent_files.clone(),
			current_theme: workspace.current_theme.clone(),
		}
	};

	let appdata_dir = get_documents_dir().join(USER_DATA_DIR);
	fs::create_dir_all(&appdata_dir).expect("Could not create appdata directory");
	let userdata_path = appdata_dir.join("userdata.json");

	match serde_json::to_string_pretty(&user_data) {
		Ok(json_content) => fs::write(userdata_path, json_content)
			.map_err(|e| format!("Failed to save userdata: {}", e)),
		Err(e) => Err(format!("Failed to serialize userdata: {}", e)),
	}
}

pub fn fetch_document_from_disk(tab_data: Tab) -> Option<MarkdownFileData> {
	// let id = tab_data.id;
	let title = tab_data.title;

	// Get the path of the document using title
	let trove_dir = get_trove_dir(TROVE_DIR);
	let file_path = trove_dir.join(format!("{}.md", title));

	// Check if the file exists
	if !file_path.exists() {
		// If the file does not exist, return None
		log::error!("File not found!");
		return None;
	}

	// Read the file content using the file path
	match fs::read_to_string(&file_path) {
		Ok(content) => {
			let documentdata = MarkdownFileData {
				id: tab_data.id,
				title,
				content,
			};
			Some(documentdata)
		} // If there is an error in reading the file, return the error
		Err(_e) => None,
	}
}

pub async fn retrieve_cached_document(app: AppHandle, tab_data: Tab) -> Option<MarkdownFileData> {
	let title = tab_data.title;
	let temp_app = app.clone();
	let state = &temp_app.state::<AppState>();

	// Acquire lock on the workspace
	let workspace = state.workspace.read().await;
	let tab_content = workspace.documents.get(&tab_data.id);
	if tab_content.is_none() {
		log::error!("Failed to load from cache!.");
		return None;
	}
	let content = tab_content.unwrap().contents.clone();
	Some(MarkdownFileData {
		id: tab_data.id,
		title,
		content,
	})
}

/// Gets the document content requested of the current tab and emits
/// it to the frontend.
// TODO: Reduce the unnecessary data cloning, like only document contents is to be sent, so pass
// only the contents and avoid cloning other variables.
pub async fn send_document_content(maybe_current_tab_data: Option<Tab>, app: AppHandle) {
	if maybe_current_tab_data.is_none() {
		log::warn!("Failed to get tab data!");
		return;
	}

	let current_tab_data = maybe_current_tab_data.unwrap();
	let mut maybe_document_data =
		retrieve_cached_document(app.clone(), current_tab_data.clone()).await;
	if maybe_document_data.is_none() {
		log::error!("Cache Miss! Loading document content from storage.");
		let tab_content = cache_document_data(app.clone(), &current_tab_data).await;
		if tab_content.is_none() {
			log::error!("Failed to get document content.");
			return;
		}
		let file_contents = tab_content.unwrap().contents.clone();
		maybe_document_data = Some(MarkdownFileData {
			id: current_tab_data.id,
			title: current_tab_data.title,
			content: file_contents,
		});
	}

	if maybe_document_data.is_none() {
		log::warn!("Failed to load document data!");
		return;
	}

	let markdown_content = maybe_document_data.unwrap().content;

	// Update the current content on the screen.
	let emit_error = app.emit("current_editor_content", markdown_content);
	if emit_error.is_err() {
		log::error!("Failed to emit current_editor_content!");
	}
}

pub async fn cache_document_data(
	app: AppHandle,
	current_tab_data: &Tab,
) -> Option<Arc<DocumentContent>> {
	let temp_app = app.clone();
	let state = &temp_app.state::<AppState>();

	let maybe_document_data = fetch_document_from_disk(current_tab_data.clone());
	if let Some(document_data) = maybe_document_data {
		let tab_content = Arc::new(DocumentContent {
			title: document_data.title,
			contents: document_data.content,
		});

		// Cache the document content on memory
		state
			.workspace
			.write()
			.await
			.documents
			.insert(current_tab_data.id.clone(), tab_content.clone());
		Some(tab_content)
	} else {
		log::error!("Failed to cache document data, file not on disk!");
		None
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

	drop(workspace); // drop workspace to avoid deadlock.

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

pub async fn save_document_helper(
	state: &State<'_, AppStateInner>,
	document_data: MarkdownFileData,
) {
	let trove_dir = get_trove_dir(TROVE_DIR);
	let safe_filename = sanitize_filename::sanitize(format!("{}.md", document_data.title));
	let file_path = trove_dir.join(&safe_filename);

	// Get the old title (Assuming the title of the document has been changed.)
	let old_title = {
		let tab_switcher = state.tab_switcher.write().await;

		tab_switcher
			.tabs
			.get(&document_data.id)
			.map(|tab| tab.title.clone())
			.unwrap_or_else(|| String::from(DEFAULT_NOTE_TITLE))
	};

	let mut workspace = state.workspace.write().await;

	// Update the recent files.
	if let Some(doc) = workspace
		.recent_files
		.iter_mut()
		.find(|doc| doc.id == document_data.id)
	{
		doc.title = document_data.title.clone();
	} else {
		workspace.recent_files.push(FileInfo {
			id: document_data.id.clone(),
			title: document_data.title.clone(),
			path: file_path.clone(),
		});
	}

	// Create the new TabDocument( tab contents ), which will be replaced by old TabDocument.
	let new_doc = Arc::new(DocumentContent {
		title: document_data.title,
		contents: document_data.content,
	});
	workspace
		.documents
		.insert(document_data.id, new_doc.clone());

	drop(workspace);

	// Get the file path to check if there was a change in title or not.
	let old_path = trove_dir.join(sanitize_filename::sanitize(format!("{}.md", old_title)));

	// if the title has changed, delete the old file
	if old_path != file_path && old_path.exists() {
		let delete_file_error =
			fs::remove_file(old_path).map_err(|e| format!("Failed to delete old file: {}", e));
		if delete_file_error.is_err() {
			log::error!("{}", delete_file_error.unwrap_err());
		}
	}

	// Now write the new file.
	let _ = if let Err(e) = fs::write(&file_path, &new_doc.contents) {
		Err(format!("Failed to write file: {}", e))
	} else {
		Ok(file_path.to_string_lossy().to_string())
	};
}

/// Generate a path that is not conflicting by incrementing a counter at file end
pub fn generate_available_path(path: PathBuf) -> PathBuf {
	if !path.exists() {
		return path;
	}
	let suffix = path
		.extension()
		.map(|ext| format!(".{}", ext.to_string_lossy()))
		.unwrap_or("".to_string());
	let prefix = path
		.file_stem()
		.unwrap_or_else(|| panic!("Unable to read path: {}", path.display()));
	let mut prefix_without_num = prefix
		.to_string_lossy()
		.to_string()
		.trim_end_matches(|c: char| c.is_ascii_digit())
		.to_string();
	if prefix.len() == prefix_without_num.len() && !prefix_without_num.ends_with(' ') {
		prefix_without_num.push(' ');
	}
	let mut num = 1;
	loop {
		let new_path = path.with_file_name(format!("{} {}{}", prefix_without_num, num, suffix));
		if !new_path.exists() {
			return new_path;
		}
		num += 1;
	}
}
