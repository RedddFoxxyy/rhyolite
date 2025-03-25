use serde::{Deserialize, Serialize};
use std::fs::*;
use std::path::PathBuf;

use crate::app_state::{CommandRegistrar, CommandRegistry};

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

// #[allow(dead_code)]
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct ThemePath {
//     name: String,
//     path: PathBuf,
// }

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
