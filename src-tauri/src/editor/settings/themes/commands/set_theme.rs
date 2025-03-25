use tauri::{AppHandle, Emitter, Manager};
use tokio::fs::read_to_string;

use crate::{
	app_state::AppState,
	editor::settings::themes::{Theme, ThemeCommands, find_file},
};

impl ThemeCommands {
	// TODO: Add condition to load an external loaded theme, that
	// is theme which is not a default theme.
	pub async fn set_theme(app: AppHandle, payload: Option<String>) {
		log::info!("init set_theme");
		// log::info!(r#"payload is: ()"#, payload);
		let Some(payload) = payload else {
			log::warn!("Invalid call to set_theme! payload is");
			return;
		};

		if let Ok(theme_name) = serde_json::from_str::<String>(&payload) {
			let resource_dir = app.path().resource_dir().unwrap();
			let themes_dir = resource_dir.join("themes");
			let theme_file_name = format!("{}.toml", theme_name);
			let theme_file_name_str = theme_file_name.as_str();
			log::info!("Setting theme to {}", theme_file_name);
			let theme_exists = find_file(&themes_dir, theme_file_name_str).await;
			if theme_exists.is_none() {
				log::error!("{} theme does not exists or is not installed!", &theme_name);
			}
			let theme_file_content = read_to_string(theme_exists.unwrap()).await.unwrap();
			let theme_result = toml::from_str::<Theme>(theme_file_content.as_str());
			if theme_result.is_err() {
				log::error!("Failed to deserialize the theme content from theme file.");
			}
			let theme = theme_result.unwrap();

			let app_ref = app.clone();
			let state = app_ref.state::<AppState>();

			let mut workspace = state.workspace.write().await;

			workspace.current_theme = theme.clone();

			let _ = app.emit("update_current_theme", theme);
		} else {
			log::error!("Invalid payload.");
		}
	}

	pub async fn get_current_theme(app: AppHandle, _payload: Option<String>) {
		log::info!("init get_current_theme");

		let app_ref = app.clone();
		let state = app_ref.state::<AppState>();

		let workspace = state.workspace.read().await;
		let current_theme = workspace.current_theme.clone();
		let _ = app.emit("update_current_theme", current_theme);
	}
}

// pub async fn set_theme_helper(app: AppHandle, theme_name: String) {
//     let resource_dir = app.path().resource_dir().unwrap();
//     let themes_dir = resource_dir.join("themes");
//     let theme_file_name = format!("{}.json", theme_name);
//     let theme_file_name_str = theme_file_name.as_str();
//     let theme_exists = find_file(&themes_dir, theme_file_name_str).await;
//     if theme_exists.is_none() {
//         log::error!("{} theme does not exists or is not installed!", &theme_name);
//     }
//     let theme_file_content = read_to_string(theme_exists.unwrap()).await.unwrap();
//     let theme_result = serde_json::from_str::<Theme>(theme_file_content.as_str());
//     if theme_result.is_err() {
//         log::error!("Failed to deserialize the theme content from theme file.");
//     }
//     let theme = theme_result.unwrap();

//     let app_ref = app.clone();
//     let state = app_ref.state::<AppState>();

//     let mut workspace = state.workspace.write().await;

//     workspace.current_theme = theme.clone();

//     let _ = app.emit("update_current_theme", theme);
// }
