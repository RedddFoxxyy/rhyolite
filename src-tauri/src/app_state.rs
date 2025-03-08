//! # App State.
//! Stores the current state and defines core skeleton of the app.
//!
//! All the required global statics are declared in this module.

use std::{
    collections::HashMap,
    fs,
    future::Future,
    path::PathBuf,
    pin::Pin,
    // sync::{Mutex, RwLock},
};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri::async_runtime::{Mutex, RwLock};
use uuid::Uuid;

use crate::editor::{
    io::{get_documents_dir, get_trove_dir},
    settings::themes::Theme,
};

// TODO: If you find any code in the code base, that uses
// string "Untitled_Trove" instead of this constant, replace it with
// this constant!
/// Name of the Default Trove used by the app!
pub const TROVE_DIR: &str = "Untitled_Trove";

// TODO: If you find any code in the code base, that uses
// string "Untitled" instead of this constant, replace it with
// this constant!
/// Name of the Default Note Title used by the app!
pub const DEFAULT_NOTE_TITLE: &str = "Untitled";

///DocumentData struct, datatype that stores id, title and content of the document.
#[derive(Serialize, Deserialize, Clone)]
pub struct DocumentData {
    pub id: String,
    pub title: String,
    pub content: String,
}

///Tab struct, used to store order(index of the tab), id of the document and title of the document.
#[derive(Serialize, Deserialize, Clone)]
pub struct Tab {
    pub id: String,
    pub title: String,
}

///Userdata Struct, used to store the userdata, like last open tab and all the open tabs.
#[derive(Serialize, Deserialize, Clone)]
pub struct UserData {
    pub tabs: Vec<Tab>,
    pub last_open_tab: String,
    pub recent_files: Vec<FileInfo>,
    pub current_theme: Theme,
}

#[derive(Default, Clone)]
pub struct TabManager {
    pub tabs: IndexMap<String, Tab>,
    pub current_tab_id: Option<String>,
}

#[allow(dead_code)]
pub struct CommandItem {
    pub name: String,
    pub action: CommandAction,
}

#[derive(Default)]
pub struct CommandRegistry {
    // TODO: indexmap or hashmap ?
    pub commands: HashMap<String, CommandItem>,
}

impl CommandRegistry {
    pub fn add_command(&mut self, name: String, action: CommandAction) {
        self.commands
            .insert(name.clone(), CommandItem { name, action });
    }
}

pub trait CommandRegistrar {
    fn register_commands(registry: &mut CommandRegistry);
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Document {
    pub path: PathBuf,
    pub title: String,
    pub contents: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
    pub id: String,
    pub title: String,
    pub path: PathBuf,
}

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FileManager {
    pub documents: Vec<Document>,
    pub recent_files: Vec<FileInfo>,
    pub current_theme: Theme,
}

#[derive(Default)]
pub struct AppStateInner {
    // TODO: Implement getters and setters for individual components of
    // the AppStateInner type so that we do not have to write same code
    // of getting state values again and again.
    //
    // Q: Should the TabManager have an rwlock or should the elements in
    // TabManager have RwLock
    pub tab_switcher: RwLock<TabManager>,
    pub command_registry: Mutex<CommandRegistry>,
    pub workspace: RwLock<FileManager>,
}

impl AppStateInner {
    pub fn load() -> Result<Self, String> {
        // Get the path of the userdata.json file
        log::debug!("load_last_open_tabs init");
        let appdata_dir = get_documents_dir().join("appdata");
        let userdata_path = appdata_dir.join("userdata.json");

        // Check if userdata.json exists
        if userdata_path.exists() {
            // Read and deserialize the UserData
            match fs::read_to_string(&userdata_path) {
                Ok(content) => {
                    // Deserialize the UserData using serde_json
                    match serde_json::from_str::<UserData>(&content) {
                        Ok(user_data) => {
                            let recent_files = user_data.recent_files.clone();

                            let current_tab_id = Some(user_data.last_open_tab.clone());

                            let current_theme = user_data.current_theme.clone();

                            let tabs = user_data
                                .tabs
                                .iter()
                                .map(|d| (d.id.to_string(), d.clone()))
                                .collect();

                            return Ok(Self {
                                tab_switcher: RwLock::new(TabManager {
                                    current_tab_id,
                                    tabs,
                                }),
                                workspace: FileManager {
                                    documents: Vec::new(),
                                    recent_files,
                                    current_theme,
                                    // ..Default::default()
                                }
                                .into(),
                                ..Default::default()
                            });
                        }
                        Err(e) => {
                            // If deserialization fails, log the error and delete the file
                            log::warn!("Failed to deserialize userdata: {}. Deleting the file.", e);

                            // Attempt to delete the problematic userdata file
                            if let Err(delete_err) = fs::remove_file(&userdata_path) {
                                log::error!(
                                    "Failed to delete corrupted userdata file: {}",
                                    delete_err
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    // If reading the file fails, log the error
                    log::warn!(
                        "Failed to read userdata file: {}. Proceeding with default.",
                        e
                    );
                }
            }
        }

        // If userdata.json doesn't exist, load all markdown files from the trove directory
        let trove_dir = get_trove_dir(TROVE_DIR);
        let mut tabs = IndexMap::new();
        let mut recent_files = Vec::new();
        let mut current_tab_id = None;

        // Read all .md files from the trove directory
        if let Ok(entries) = fs::read_dir(&trove_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                if let Some(extension) = entry.path().extension() {
                    if extension == "md" {
                        if let Some(stem) = entry.path().file_stem().and_then(|s| s.to_str()) {
                            let id = Uuid::new_v4().to_string();
                            let title = stem.to_string();

                            let tab = Tab {
                                id: id.clone(),
                                title: title.clone(),
                            };

                            tabs.insert(id.clone(), tab);
                            recent_files.push(FileInfo {
                                id: id.clone(),
                                title,
                                path: entry.path(),
                            });

                            if current_tab_id.is_none() {
                                current_tab_id = Some(id);
                            }
                        }
                    }
                }
            }
        }

        // If no files were found, create a new empty file
        if tabs.is_empty() {
            let id = Uuid::new_v4().to_string();
            let title = DEFAULT_NOTE_TITLE.to_string();

            let tab = Tab {
                id: id.clone(),
                title: title.clone(),
            };

            // Create empty file
            let file_path = trove_dir.join("Untitled.md");
            fs::write(&file_path, "").map_err(|e| format!("Failed to create empty file: {}", e))?;

            tabs.insert(id.clone(), tab);
            recent_files.push(FileInfo {
                id: id.clone(),
                title,
                path: file_path,
            });
            current_tab_id = Some(id);
        }

        Ok(Self {
            tab_switcher: RwLock::new(TabManager {
                current_tab_id,
                tabs,
            }),
            workspace: FileManager {
                documents: Vec::new(),
                recent_files,
                current_theme: Theme::default(),
            }
            .into(),
            command_registry: Mutex::new(CommandRegistry::default()),
        })
    }
}

pub type AppState = AppStateInner;
// Todo: Implement Async for the CommandActions.
// I am also removing the Arc and Mutex for the command actions for now since it is
// not required. If needed we can wrap it again in arc and mutex.
pub type CommandAction = Box<
    dyn FnMut(AppHandle, Option<String>) -> Pin<Box<dyn Future<Output = ()> + Send>>
        + Send
        + Sync
        + 'static,
>;
