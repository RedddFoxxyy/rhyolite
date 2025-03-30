use crate::{
	app_state::AppState,
	editor::{
		io::commands::get_document_content::send_document_content,
		tabs::{TabCommands, update_tabs_state},
	},
};
use log::*;
use tauri::{AppHandle, Manager};

impl TabCommands {
	pub async fn switch_tab(app: AppHandle, payload: Option<String>) {
		let Some(payload) = payload else {
			warn!("Invalid call to switch_tab");
			return;
		};
		if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&payload) {
			if let Some(tab_id) = json_value.get("tabId").and_then(|v| v.as_str()) {
				let temp_app = app.clone();
				let state = temp_app.state::<AppState>();

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
}
