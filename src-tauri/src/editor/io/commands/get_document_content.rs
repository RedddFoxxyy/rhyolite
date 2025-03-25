use crate::{
	app_state::{MarkdownFileData, TROVE_DIR, Tab},
	editor::{
		io::{IOCommands, get_trove_dir},
		markdown_handler,
	},
};
use std::fs;
use tauri::{AppHandle, Emitter};

impl IOCommands {
	//TODO: Cleanup unused variables.
	pub async fn get_document_content(app: AppHandle, payload: Option<String>) {
		let Some(payload) = payload else {
			log::warn!("Invalid call to get_document_content");
			return;
		};

		if let Ok(tab_data) = serde_json::from_str::<Tab>(&payload) {
			send_document_content(Some(tab_data), app.clone());
		}
	}
}

pub fn get_document_content_helper(tab_data: Tab) -> Option<MarkdownFileData> {
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

/// Gets the document content requested of the current tab and emits
/// it to the frontend.
pub fn send_document_content(current_tab_data: Option<Tab>, app: AppHandle) {
	if current_tab_data.is_none() {
		log::warn!("Failed to get tab data!");
		return;
	}

	let document_data = get_document_content_helper(current_tab_data.unwrap());
	if document_data.is_none() {
		log::warn!("Failed to load document data!");
		return;
	}
	let html_output = markdown_handler::markdown_to_html(&document_data.unwrap().content);

	// Update the current content on the screen.
	let emit_error = app.emit("current_editor_content", html_output);
	if emit_error.is_err() {
		log::error!("Failed to emit current_editor_content!");
	}
}
