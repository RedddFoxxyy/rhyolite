use crate::{
	app_state::AppState,
	editor::{
		io::{IOCommands, commands::get_document_content::send_document_content},
		tabs::update_tabs_state,
	},
};
use tauri::{AppHandle, Manager};

impl IOCommands {
	pub async fn init_frontend_state(app: AppHandle, _payload: Option<String>) {
		log::debug!("load_last_open_tabs init");
		let temp_app = app.clone();
		let state = &temp_app.state::<AppState>();

		update_tabs_state(app.clone()).await;
		let tab_switcher = state.tab_switcher.read().await;
		let current_tab_id = tab_switcher.current_tab_id.clone();
		if current_tab_id.is_none() {
			log::error!("Failed to get current tab id");
			return;
		}
		let current_tab_data = tab_switcher.tabs.get(&current_tab_id.unwrap()).cloned();
		send_document_content(current_tab_data, app);
	}
}
