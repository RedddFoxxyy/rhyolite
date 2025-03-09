use crate::editor::settings::themes::{ThemeCommands, ThemeListItem};
use tauri::{AppHandle, Emitter, Manager};
use tokio::fs::{read_dir, read_to_string};
use toml::Value;

impl ThemeCommands {
    pub async fn get_loaded_themes(app: AppHandle, _payload: Option<String>) {
        log::info!("Init get_loaded_themes");

        let resource_dir = app.path().resource_dir().unwrap();
        let themes_dir = resource_dir.join("themes");
        let mut themes_list: Vec<ThemeListItem> = Vec::new();

        if let Ok(mut entries) = read_dir(&themes_dir).await {
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

// pub async fn get_loaded_themes(app: AppHandle, _payload: Option<String>) {
//     log::info!("Init get_loaded_themes");

//     let resource_dir = app.path().resource_dir().unwrap();
//     let themes_dir = resource_dir.join("themes");
//     let mut themes_list: Vec<String> = Vec::new();
//     if let Ok(entries) = read_dir(&themes_dir) {
//         for entry in entries.filter_map(|e| e.ok()) {
//             if let Some(extension) = entry.path().extension() {
//                 if extension == "json" {
//                     if let Some(stem) = entry.path().file_stem().and_then(|s| s.to_str()) {
//                         themes_list.push(stem.to_string());
//                     }
//                 }
//             }
//         }
//         // println!("{:#?}", themes_list);
//         let _ = app.emit("themes_list", themes_list);
//     } else {
//         log::error!("Failed to get the resource dir.");
//     }
// }
