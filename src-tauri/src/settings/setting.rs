use crate::editor::io;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct SettingCategory {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub setting: Vec<Setting>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Setting {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub select: Option<Vec<String>>,
    #[serde(default)]
    pub check: Option<bool>,
    #[serde(default)]
    pub selected: Option<String>,
}

fn get_settings_path() -> PathBuf {
    let mut path = io::get_documents_dir();
    path.push("settings.json");
    path
}

pub fn check_settings_json() -> Result<(), String> {
    let settings_path = get_settings_path();

    if !settings_path.exists() {
        let default_settings = r#"[
  {
    "title": "General",
    "description": "General settings",
    "setting": [
      {
        "title": "Language",
        "description": "Language settings",
        "select": [
          "English"
        ],
        "check": null,
        "selected": "English"
      }
    ]
  },
  {
    "title": "Appearance",
    "description": "Appearance settings",
    "setting": [
      {
        "title": "Theme",
        "description": "Theme settings",
        "select": [
          "Light",
          "Dark",
          "Custom"
        ],
        "check": null,
        "selected": "Dark"
      },
      {
        "title": "App Font",
        "description": "Select a font to use in the app",
        "select": [
          "Roboto",
          "Arial",
          "Times New Roman",
          "Courier New"
        ],
        "check": null,
        "selected": "Times New Roman"
      },
      {
        "title": "App Font Size",
        "description": "Select a font size to use in the app",
        "select": [
          "Small",
          "Medium",
          "Large"
        ],
        "check": null,
        "selected": "Medium"
      },
      {
        "title": "App Font Weight",
        "description": "Select a font weight to use in the app",
        "select": [
          "Light",
          "Regular",
          "Bold"
        ],
        "check": null,
        "selected": "Regular"
      },
      {
        "title": "Editor Font",
        "description": "Select a font to use in the editor",
        "select": [
          "Roboto",
          "Arial",
          "Times New Roman",
          "Courier New"
        ],
        "check": null,
        "selected": "Roboto"
      },
      {
        "title": "Editor Font Size",
        "description": "Select a font size to use in the editor",
        "select": [
          "Small",
          "Medium",
          "Large"
        ],
        "check": null,
        "selected": "Medium"
      },
      {
        "title": "Editor Font Weight",
        "description": "Select a font weight to use in the editor",
        "select": [
          "Light",
          "Regular",
          "Bold"
        ],
        "check": null,
        "selected": "Regular"
      }
    ]
  },
  {
    "title": "Shortcuts",
    "description": "Shortcuts settings",
    "setting": [
      {
        "title": "Delete Document",
        "description": "Delete the current document",
        "select": [
          "Ctrl+D",
          "Cmd+D",
          "Delete"
        ],
        "check": null,
        "selected": "meta+backspace"
      },
      {
        "title": "Close Tab",
        "description": "Close the current tab",
        "select": [
          "Ctrl+C",
          "Cmd+W",
          "Alt+F4"
        ],
        "check": null,
        "selected": "meta+w"
      },
      {
        "title": "New Document",
        "description": "Create a new document",
        "select": [
          "Ctrl+N",
          "Cmd+N",
          "Alt+N"
        ],
        "check": false,
        "selected": "meta+n"
      },
      {
        "title": "Toggle Toolbar",
        "description": "Show/hide the toolbar",
        "select": [
          "Ctrl+T",
          "Cmd+T",
          "F11"
        ],
        "check": null,
        "selected": "meta+b"
      },
      {
        "title": "Cycle Tabs",
        "description": "Switch between tabs",
        "select": [
          "Ctrl+Tab",
          "Cmd+`",
          "Alt+Tab"
        ],
        "check": null,
        "selected": "meta+a"
      },
      {
        "title": "Force Close Tab",
        "description": "Force close the current tab",
        "select": [
          "Ctrl+Alt+C",
          "Cmd+Alt+W",
          "Shift+Alt+F4"
        ],
        "check": null,
        "selected": "ctrl+c"
      },
      {
        "title": "Go to First Tab",
        "description": "Switch to the first tab",
        "select": [
          "Ctrl+1",
          "Cmd+1",
          "Alt+1"
        ],
        "check": null,
        "selected": "ctrl+1"
      },
      {
        "title": "Go to Last Tab",
        "description": "Switch to the last tab",
        "select": [
          "Ctrl+9",
          "Cmd+9",
          "Alt+9"
        ],
        "check": null,
        "selected": "ctrl+2"
      },
      {
        "title": "Command Palette",
        "description": "Open the command palette",
        "select": [
          "Ctrl+P",
          "Cmd+P",
          "F1"
        ],
        "check": null,
        "selected": "ctrl+3"
      }
    ]
  },
  {
    "title": "Sync",
    "description": "Sync settings",
    "setting": []
  }
]"#;
        fs::write(&settings_path, default_settings)
            .map_err(|e| format!("Failed to write settings file: {}", e))?;
    }

    let settings_str = fs::read_to_string(&settings_path)
        .map_err(|e| format!("Failed to read settings file: {}", e))?;
    serde_json::from_str::<Vec<SettingCategory>>(&settings_str)
        .map_err(|e| format!("Failed to parse settings file: {}", e))?;

    Ok(())
}
#[tauri::command]
pub fn save_setting(title: String, value: String) -> Result<(), String> {
    check_settings_json()?;

    let settings_str = fs::read_to_string(get_settings_path())
        .map_err(|e| format!("Failed to read settings file: {}", e))?;

    let mut settings: Vec<SettingCategory> = serde_json::from_str(&settings_str)
        .map_err(|e| format!("Failed to parse settings file: {}", e))?;

    for section in &mut settings {
        for setting in &mut section.setting {
            if setting.title == title {
                setting.selected = Some(value);
                let json = serde_json::to_string_pretty(&settings)
                    .map_err(|e| format!("Failed to serialize settings: {}", e))?;
                fs::write(get_settings_path(), json)
                    .map_err(|e| format!("Failed to write settings file: {}", e))?;
                return Ok(());
            }
        }
    }

    Err(format!("No setting found with title: {}", title))
}

#[tauri::command]
pub fn get_setting_value(title: String) -> Result<Setting, String> {
    check_settings_json()?;

    let settings_str = fs::read_to_string(get_settings_path())
        .map_err(|e| format!("Failed to read settings file: {}", e))?;

    let settings: Vec<SettingCategory> = serde_json::from_str(&settings_str)
        .map_err(|e| format!("Failed to parse settings file: {}", e))?;

    for section in &settings {
        for setting in &section.setting {
            if setting.title == title {
                return Ok(setting.clone());
            }
        }
    }

    Err(format!("No setting found with title: {}", title))
}

#[tauri::command]
pub fn get_all_settings() -> Result<Vec<SettingCategory>, String> {
    check_settings_json()?;
    let settings_str = fs::read_to_string(get_settings_path())
        .map_err(|e| format!("Failed to read settings file: {}", e))?;

    let settings: Vec<SettingCategory> = serde_json::from_str(&settings_str)
        .map_err(|e| format!("Failed to parse settings file: {}", e))?;

    Ok(settings)
}

#[tauri::command]
pub fn toggle_check(title: String) -> Result<(), String> {
    let settings_str = fs::read_to_string(get_settings_path())
        .map_err(|e| format!("Failed to read settings file: {}", e))?;

    let mut settings: Vec<SettingCategory> = serde_json::from_str(&settings_str)
        .map_err(|e| format!("Failed to parse settings file: {}", e))?;

    for section in &mut settings {
        for setting in &mut section.setting {
            if setting.title == title {
                setting.check = Some(!setting.check.unwrap_or(false));
                let json = serde_json::to_string_pretty(&settings)
                    .map_err(|e| format!("Failed to serialize settings: {}", e))?;
                fs::write(get_settings_path(), json)
                    .map_err(|e| format!("Failed to write settings file: {}", e))?;
                return Ok(());
            }
        }
    }

    Err(format!("No setting found with title: {}", title))
}
