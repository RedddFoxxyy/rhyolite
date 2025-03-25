use tauri::{AppHandle, Manager};

use crate::{
	app_state::AppState,
	editor::settings::themes::{Theme, ThemeCommands},
};

impl ThemeCommands {
	pub async fn reset_theme(app: AppHandle, payload: Option<String>) {
		log::info!("init reset_theme");
		let Some(payload) = payload else {
			log::warn!("Invalid call to switch_tab");
			return;
		};

		if let Ok(theme) = serde_json::from_str::<Theme>(&payload) {
			let app_ref = app.clone();
			let state = app_ref.state::<AppState>();

			let mut workspace = state.workspace.write().await;
			workspace.current_theme = theme;
		} else {
			log::error!("Failed to reset theme as deseriaization of data failed.");
		}
	}
}
