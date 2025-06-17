use serde::{Deserialize, Serialize};
use std::{fs::read_dir, path::PathBuf};
use tokio::fs::read_to_string;
use toml::Value;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Theme {
	pub info: ThemeInfo,
	pub colors: Colors,
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
	pub text: String,
	pub subtext2: String,
	pub subtext1: String,
	pub subtext0: String,
	pub overlay2: String,
	pub overlay1: String,
	pub overlay0: String,
	pub surface2: String,
	pub surface1: String,
	pub surface0: String,
	pub base: String,
	pub crust: String,
	pub mantle: String,
	pub accent: String,
	pub highlight: String,
	pub border: String,
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
