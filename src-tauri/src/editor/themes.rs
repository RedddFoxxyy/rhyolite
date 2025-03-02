use serde::{Deserialize, Serialize};

use crate::app_state::{CommandRegistrar, CommandRegistry};

#[derive(Serialize, Deserialize, Clone)]
pub struct Theme {
    name: String,
    colorscheme: String,
    colors: Colors,
}
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

#[derive(Serialize, Deserialize, Clone)]
struct Colors {
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
    fn register_commands(registry: &mut CommandRegistry) {}
}
