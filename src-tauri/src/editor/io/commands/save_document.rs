use crate::{
	app_state::{
		AppState, AppStateInner, FileInfo, MarkdownFileData, DocumentContent, DEFAULT_NOTE_TITLE, TROVE_DIR
	},
	editor::{
		io::{get_trove_dir, IOCommands},
		markdown_handler,
	},
};
use std::{fs, sync::Arc};
use tauri::{AppHandle, Manager, State};

impl IOCommands {
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

pub async fn save_document_helper(
	state: &State<'_, AppStateInner>,
	document_data: MarkdownFileData,
) {
	let trove_dir = get_trove_dir(TROVE_DIR);
	let markdown_content = markdown_handler::html_to_markdown(&document_data.content);
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
		contents: document_data.content
	});
	workspace.documents.insert(document_data.id, new_doc.clone());
	
	drop(workspace);

	// Get the file path to check if there was a change in title or not.
	let old_path = trove_dir.join(sanitize_filename::sanitize(format!("{}.md", old_title)));

	// if the title has changed, delete the old file
	if old_path != file_path && old_path.exists() {
		let delete_file_error = fs::remove_file(old_path).map_err(|e| format!("Failed to delete old file: {}", e));
		if delete_file_error.is_err() {
			log::error!("{}", delete_file_error.unwrap_err());
		}
	}

	// Now write the new file.
	let _ = if let Err(e) = fs::write(&file_path, markdown_content) {
		Err(format!("Failed to write file: {}", e))
	} else {
		Ok(file_path.to_string_lossy().to_string())
	};
}
