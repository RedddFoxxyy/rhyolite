use crate::{
	app_state::{AppState, MarkdownFileData, Tab, TabDocument, TROVE_DIR},
	editor::{
		io::{get_trove_dir, IOCommands},
		markdown_handler,
	},
};
use std::{fs, sync::Arc};
use tauri::{AppHandle, Emitter, Manager};

impl IOCommands {
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

pub async fn retrieve_cached_document(app:AppHandle, tab_data: Tab) -> Option<MarkdownFileData> {
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
	let mut maybe_document_data = retrieve_cached_document(app.clone(), current_tab_data.clone()).await;
	if maybe_document_data.is_none() {
		log::error!("Cache Miss! Loading document content from storage.");
		let tab_content = cache_document_data(app.clone(), &current_tab_data).await;
		if tab_content.is_none() {
			log::error!("Failed to get document content.");
			return;
		}
		let file_contents = tab_content.unwrap().contents.clone();
		maybe_document_data = Some( MarkdownFileData {
			id: current_tab_data.id,
			title: current_tab_data.title,
			content: file_contents
		});
	}
	
	if maybe_document_data.is_none() {
		log::warn!("Failed to load document data!");
		return;
	}
	
	let html_output = markdown_handler::markdown_to_html(&maybe_document_data.unwrap().content);

	// Update the current content on the screen.
	let emit_error = app.emit("current_editor_content", html_output);
	if emit_error.is_err() {
		log::error!("Failed to emit current_editor_content!");
	}
}

pub async fn cache_document_data(app: AppHandle, current_tab_data: &Tab) -> Option<Arc<TabDocument>> {
	let temp_app = app.clone();
	let state = &temp_app.state::<AppState>();
	
	let maybe_document_data = fetch_document_from_disk(current_tab_data.clone());
	if let Some(document_data) = maybe_document_data {
		let tab_content = Arc::new(TabDocument {
			title: document_data.title,
			contents: document_data.content	
		});
		
		// Cache the document content on memory
		state.workspace.write().await.documents.insert(current_tab_data.id.clone(), tab_content.clone());
		Some(tab_content)
	} else {
		log::error!("Failed to cache document data, file not on disk!");
		None
	}
}
