use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use tokio::fs::read_to_string;
use toml::Value;
use std::{fs::read_dir, path::PathBuf};

use crate::app_state::{AppState, CommandRegistrar, CommandRegistry};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Theme {
	info: ThemeInfo,
	colors: Colors,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ThemeInfo {
	name: String,
	author: String,
	themetype: ThemeType,
	colorscheme: ColorScheme,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum ThemeType {
	#[default]
	Basic,
	Advance,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum ColorScheme {
	Light,
	#[default]
	Dark,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Colors {
	text: String,
	subtext2: String,
	subtext1: String,
	subtext0: String,
	overlay2: String,
	overlay1: String,
	overlay0: String,
	surface2: String,
	surface1: String,
	surface0: String,
	base: String,
	crust: String,
	mantle: String,
	accent: String,
	highlight: String,
	border: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThemeListItem {
	pub filename: String, // File stem (e.g., "dark")
	pub name: String,     // Display name from TOML (e.g., "My Dark Theme")
}

impl Theme {
	pub fn default() -> Theme {
		Theme {
			info: ThemeInfo {
				name: "Default".to_string(),
				author: "Rhyolite Team".to_string(),
				themetype: ThemeType::Basic,
				colorscheme: ColorScheme::Dark,
			},
			colors: Colors {
				text: "#ffffff".to_string(),
				subtext2: "#f1f2f3".to_string(),
				subtext1: "#d8dbde".to_string(),
				subtext0: "#c2c6cb".to_string(),
				overlay2: "#acb2b8".to_string(),
				overlay1: "#969da5".to_string(),
				overlay0: "#808992".to_string(),
				surface2: "#6c757d".to_string(),
				surface1: "#596167".to_string(),
				surface0: "#464c51".to_string(),
				base: "#33373b".to_string(),
				crust: "#202325".to_string(),
				mantle: "#0d0e0f".to_string(),
				accent: "#ff4081".to_string(),
				highlight: "#ffa726".to_string(),
				border: "#424242".to_string(),
			},
		}
	}
}

pub struct ThemeCommands;
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

	pub async fn get_loaded_themes(app: AppHandle, _payload: Option<String>) {
		log::info!("Init get_loaded_themes");

		let resource_dir = app.path().resource_dir().unwrap();
		let themes_dir = resource_dir.join("themes");
		let mut themes_list: Vec<ThemeListItem> = Vec::new();

		if let Ok(mut entries) = tokio::fs::read_dir(&themes_dir).await {
			while let Ok(Some(entry)) = entries.next_entry().await {
				if let Some(extension) = entry.path().extension() {
					if extension == "toml" {
						if let Some(stem) = entry.path().file_stem().and_then(|s| s.to_str()) {
							// Read and parse the TOML file
							let content = match read_to_string(&entry.path()).await {
								Ok(c) => c,
								Err(e) => {
									log::error!(
										"Failed to read theme file {}: {}",
										entry.path().display(),
										e
									);
									continue;
								}
							};
							let value: Value = match toml::from_str(&content) {
								Ok(v) => v,
								Err(e) => {
									log::error!(
										"Failed to parse theme file {}: {}",
										entry.path().display(),
										e
									);
									continue;
								}
							};
							// Extract info.name
							if let Some(name) = value
								.get("info")
								.and_then(|info| info.get("name"))
								.and_then(|n| n.as_str())
							{
								themes_list.push(ThemeListItem {
									filename: stem.to_string(),
									name: name.to_string(),
								});
							} else {
								log::warn!(
									"Theme file {} does not have info.name",
									entry.path().display()
								);
							}
						}
					}
				}
			}
			let _ = app.emit("themes_list", themes_list);
		} else {
			log::error!("Failed to get the resource dir.");
		}
	}
}
impl CommandRegistrar for ThemeCommands {
	fn register_commands(registry: &mut CommandRegistry) {
		registry.add_command(
			"set_theme".to_string(),
			Box::new(|app, payload| Box::pin(Self::set_theme(app, payload))),
		);
		registry.add_command(
			"get_loaded_themes".to_string(),
			Box::new(|app, payload| Box::pin(Self::get_loaded_themes(app, payload))),
		);
		registry.add_command(
			"get_current_theme".to_string(),
			Box::new(|app, payload| Box::pin(Self::get_current_theme(app, payload))),
		);
		registry.add_command(
			"reset_theme".to_string(),
			Box::new(|app, payload| Box::pin(Self::reset_theme(app, payload))),
		);
	}
}

pub async fn find_file(dir: &PathBuf, target: &str) -> Option<PathBuf> {
	// read_dir returns an iterator over the directory entries
	read_dir(dir)
		.ok()?
		.filter_map(Result::ok)
		.find(|entry| {
			// file_name returns an OsString; convert it to &str to compare
			entry.file_name().to_string_lossy() == target
		})
		.map(|entry| entry.path())
}
