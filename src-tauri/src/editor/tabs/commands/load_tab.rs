use crate::{
	app_state::{AppState, Tab},
	editor::{
		io::commands::get_document_content::send_document_content,
		tabs::{TabCommands, update_tabs_state},
	},
};
use tauri::{AppHandle, Manager};

impl TabCommands {
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
}
