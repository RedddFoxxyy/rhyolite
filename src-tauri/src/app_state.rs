use std::{
    collections::HashMap,
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use uuid::Uuid;

use crate::editor::io::{get_documents_dir, get_trove_dir};

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
    pub recent_files: Vec<RecentFileInfo>,
}

#[derive(Default, Clone)]
pub struct TabSwitcher {
    pub tabs: IndexMap<String, Tab>,
    pub current_tab_id: Option<String>,
}

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
    pub fn add_command(
        &mut self,
        name: String,
        action: CommandAction,
    ) {
        self.commands.insert(
            name.clone(),
            CommandItem {
                name,
                action,
            },
        );
    }
}

pub trait CommandRegistrar {
    fn register_commands(registry: &mut CommandRegistry);
}

#[derive(Debug)]
pub struct Document {
    pub path: PathBuf,
    pub title: String,
    pub contents: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecentFileInfo {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Default)]
pub struct WorkSpace {
    pub documents: Vec<Document>,
    pub recent_files: Vec<RecentFileInfo>,
}

#[derive(Default)]
pub struct AppStateInner {
    /// TODO: Since AppState will be having complex structure, we probably don't need to lock
    /// the entire state when we only want to read a specific value.
    /// Q. Hence, shall we have
    /// 1. an AppState which itself isn't a mutex but the innermost values are mutex. Like
    ///    AppStateInner::TabSwitcher::tabs could be mutex
    /// 2. multiple states which are mutex in themselves and are registered using multiple calls to
    ///    app.manage()
    pub tab_switcher: Mutex<TabSwitcher>,
    pub command_registry: Mutex<CommandRegistry>,
    pub workspace: Mutex<WorkSpace>,
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

                            let tabs = user_data
                                .tabs
                                .iter()
                                .map(|d| (d.id.to_string(), d.clone()))
                                .collect();

                            return Ok(Self {
                                tab_switcher: Mutex::new(TabSwitcher {
                                    current_tab_id,
                                    tabs,
                                }),
                                workspace: WorkSpace {
                                    recent_files,
                                    ..Default::default()
                                }
                                .into(),
                                ..Default::default()
                            });
                        }
                        Err(e) => return Err(format!("Failed to deserialize userdata: {}", e)),
                    }
                }
                Err(e) => return Err(format!("Failed to read userdata file: {}", e)),
            }
        }

        // If userdata.json doesn't exist, load all markdown files from the trove directory
        let trove_dir = get_trove_dir("Untitled_Trove");
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
                            recent_files.push(RecentFileInfo {
                                id: id.clone(),
                                title,
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
            let title = "Untitled".to_string();

            let tab = Tab {
                id: id.clone(),
                title: title.clone(),
            };

            // Create empty file
            let file_path = trove_dir.join("Untitled.md");
            fs::write(&file_path, "").map_err(|e| format!("Failed to create empty file: {}", e))?;

            tabs.insert(id.clone(), tab);
            recent_files.push(RecentFileInfo {
                id: id.clone(),
                title,
            });
            current_tab_id = Some(id);
        }

        Ok(Self {
            tab_switcher: Mutex::new(TabSwitcher {
                current_tab_id,
                tabs,
            }),
            workspace: WorkSpace {
                documents: Vec::new(),
                recent_files,
            }
            .into(),
            command_registry: Mutex::new(CommandRegistry::default()),
        })
    }
}

pub type AppState = AppStateInner;
pub type CommandAction = Arc<Mutex<Box<dyn FnMut(AppHandle, String) + Send + 'static>>>;
