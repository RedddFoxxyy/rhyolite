use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};

use crate::app_state::{AppState, CommandRegistrar, CommandRegistry};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Theme {
    name: String,
    colorscheme: String,
    colors: Colors,
}

#[allow(dead_code)]
pub struct ThemePath {
    name: String,
    path: PathBuf,
}
pub const DEFAULT_THEMES: Vec<Theme> = Vec::new();
pub const THEMES_LIST: Vec<ThemePath> = Vec::new();

impl Theme {
    pub fn default() -> Theme {
        Theme {
            name: "Default".to_string(),
            colorscheme: "dark".to_string(),
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
            },
        }
    }
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
    }
}

impl ThemeCommands {
    // TODO: Add condition to load an external loaded theme, that
    // is theme which is not a default theme.
    pub async fn set_theme(app: AppHandle, payload: Option<String>) {
        let Some(payload) = payload else {
            log::warn!("Invalid call to switch_tab");
            return;
        };

        if let Ok(theme_name) = serde_json::from_str::<String>(&payload) {
            let default_theme = &DEFAULT_THEMES;
            let maybe_theme = default_theme
                .iter()
                .find(|theme| theme.name == theme_name.clone());

            if maybe_theme.is_none() {
                log::error!("The given them {} was not found!!", theme_name);
                return;
            }

            let new_theme = maybe_theme.unwrap();

            let app_ref = app.clone();
            let state = app_ref.state::<AppState>();

            let mut workspace = state.workspace.write().await;

            workspace.current_theme = new_theme.clone();

            let _ = app.emit("update_current_theme", new_theme.clone());
        } else {
            log::error!("Invalid payload.");
        }
    }
    pub async fn get_loaded_themes(app: AppHandle, _payload: Option<String>) {
        let default_themes = &DEFAULT_THEMES;
        let ext_themes_list = &THEMES_LIST;
        let default_theme_names_iter = default_themes.iter().map(|theme| theme.name.clone());
        let default_theme_names: Vec<String> = default_theme_names_iter.collect();
        let loaded_theme_names_iter = ext_themes_list.iter().map(|theme| theme.name.clone());
        let loaded_theme_names: Vec<String> = loaded_theme_names_iter.collect();

        let final_theme_list: Vec<String> = default_theme_names
            .into_iter()
            .chain(loaded_theme_names)
            .collect();

        let _ = app.emit("themes_list", final_theme_list);
    }
}
