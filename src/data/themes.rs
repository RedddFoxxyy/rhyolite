use serde::{Deserialize, Serialize};
// use std::{
// 	fs::{self, read_dir},
// 	path::PathBuf,
// };
// use tokio::fs::read_to_string;
// use toml::Value;

include!("../build/themes_build.rs");

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThemesStore {
	pub store: Vec<Theme>,
	pub current_theme: Theme,
}
impl ThemesStore {
	pub fn default() -> ThemesStore {
		let mut theme_vec: Vec<Theme> = Vec::new();

		let mut sorted_themes_vec: Vec<_> = THEMES.iter().collect();

		// Sort the themes alphabetically by filename
		sorted_themes_vec.sort_by(|a, b| a.0.cmp(b.0));

		for (file_name, toml_str) in sorted_themes_vec {
			let theme: Theme = toml::from_str(toml_str).unwrap_or_else(|e| panic!("Error parsing {file_name}: {e}"));

			theme_vec.push(theme);
		}

		ThemesStore {
			store: theme_vec,
			current_theme: Theme::default(),
		}
	}

	pub fn change_current_theme(&mut self, theme_name: &String) {
		// Had to do this workaround cause the button props on_click property only takes a &String.
		if let Some(index) = self.store.iter().position(|theme| &theme.info.name == theme_name) {
			self.current_theme = self.store.get(index).unwrap().clone();
		}
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

// pub fn new_theme_store() -> (Vec<Theme>, Vec<String>) {
// 	let mut default_theme_store: Vec<Theme> = Vec::new();
// 	let mut theme_list: Vec<String> = Vec::new();

// 	let themes_path = "../../app_themes";

// 	for themefile in read_dir(themes_path).unwrap() {
// 		let theme_file_path = themefile.unwrap().path();

// 		if theme_file_path
// 			.extension()
// 			.and_then(|s| s.to_str())
// 			.map(|ext| ext.eq_ignore_ascii_case("toml"))
// 			.unwrap_or(false)
// 		{
// 			let theme_content = fs::read_to_string(&theme_file_path).unwrap();

// 			let theme: Theme = toml::from_str(&theme_content).unwrap();
// 			let theme_name = theme.info.name.clone();
// 			default_theme_store.push(theme);
// 			theme_list.push(theme_name);
// 		}
// 	}

// 	(default_theme_store, theme_list)
// }
