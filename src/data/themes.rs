// Copyright (C) 2025  Suyog Tandel(RedddFoxxyy)
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::data::types::APP_DATA_DIR;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ThemesStore {
	pub themes_dir: PathBuf,
	pub themes_list: Vec<(String, PathBuf)>,
	pub current_theme: Theme,
}
impl ThemesStore {
	// TODO: Make this as new function and make a new default function.
	pub fn init() -> ThemesStore {
		let themes_dir = {
			let Some(data) = dirs::state_dir() else {
				eprintln!("No Data directory could be found/accessed!");
				panic!("Failed to find Data directory.")
			};
			let app_data_dir = data.join(APP_DATA_DIR);

			let themes_dir = app_data_dir.join("Themes");

			if let Err(e) = fs::create_dir_all(&themes_dir) {
				eprintln!("Failed to create App Themes directory: {}", e);
			};

			themes_dir
		};

		let themes_list_result = list_toml_names(&themes_dir);

		if let Err(e) = themes_list_result {
			log::error!("Failed to list themes in directory: {}", e);
			log::warn!("Using default theme.");
			log::warn!("No themes to select from..");
			return ThemesStore {
				themes_dir,
				themes_list: vec![],
				current_theme: Theme::default(),
			};
		}

		ThemesStore {
			themes_dir,
			themes_list: themes_list_result.unwrap(),
			current_theme: Theme::default(),
		}
	}

	pub async fn change_current_theme(&mut self, theme_path: PathBuf) {
		if let Some(theme) = self.load_theme_from_file(theme_path) {
			self.current_theme = theme.clone();
		} else {
			log::error!("The given theme does not exists or is corrupted.");
		}
	}

	// pub async fn _preview_theme(&mut self, preview: bool, theme_index: usize, original_theme: &Option<Theme>) {
	// 	if preview {
	// 		if original_theme.is_none() {
	// 			// This is the first preview, store current and switch to preview
	// 			self.current_theme = self.themes_path.get(theme_index).unwrap().clone();
	// 		} else {
	// 			// Already previewing, just switch to new preview theme
	// 			self.current_theme = self.themes_path.get(theme_index).unwrap().clone();
	// 		}
	// 	} else {
	// 		// Restore original theme
	// 		if let Some(original) = original_theme {
	// 			self.current_theme = original.clone();
	// 		}
	// 	}
	// }

	// TODO: Handle errors.
	fn load_theme_from_file(&self, path: PathBuf) -> Option<Theme> {
		let theme_content = fs::read_to_string(path).unwrap_or_default();
		toml::from_str(&theme_content).ok()
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Theme {
	pub info: ThemeInfo,
	pub colors: Colors,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ThemeInfo {
	pub name: String,
	pub author: String,
	pub themetype: ThemeType,
	pub colorscheme: ColorScheme,
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
	pub filename: String,
	pub name: String,
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

fn list_toml_names(dir: &PathBuf) -> std::io::Result<Vec<(String, PathBuf)>> {
	let mut names = Vec::new();

	for entry in fs::read_dir(dir)? {
		let entry = entry?;
		let path = entry.path();

		if path.extension().and_then(|e| e.to_str()) == Some("toml") {
			let theme = fs::read_to_string(&path).unwrap_or_default();
			if let Ok(theme) = toml::from_str::<Theme>(&theme) {
				let theme_name = theme.info.name;
				names.push((theme_name, path));
			} else {
				log::error!("{path:?} is not a theme file!")
			}
		}
	}
	names.sort_by_key(|a| a.0.to_lowercase());

	Ok(names)
}
