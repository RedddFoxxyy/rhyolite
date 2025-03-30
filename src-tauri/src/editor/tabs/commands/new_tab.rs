use crate::{
	app_state::{AppState, DEFAULT_NOTE_TITLE, MarkdownFileData, TROVE_DIR, Tab},
	editor::{
		io::{
			commands::{
				get_document_content::send_document_content, save_document::save_document_helper,
			},
			get_trove_dir, save_user_data,
		},
		tabs::{TabCommands, cleanup_deleted_files_workaround, update_tabs_state},
	},
	utils::generate_available_path,
};
use tauri::{AppHandle, Manager};
use uuid::Uuid;

impl TabCommands {
	pub async fn new_tab(app: AppHandle, _payload: Option<String>) {
		log::debug!("new_tab init");
		new_tab_helper(app.clone()).await;
	}
}

pub async fn new_tab_helper(app: AppHandle) {
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
