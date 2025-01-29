use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::editor::io::get_documents_dir;

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
    pub action: Arc<Mutex<Box<dyn FnMut(AppHandle, String) + Send + 'static>>>,
}

#[derive(Default)]
pub struct CommandRegistry {
    // TODO: indexmap or hashmap ?
    pub commands: IndexMap<String, CommandItem>,
}
impl CommandRegistry {
    pub fn add_command(
        &mut self,
        name: String,
        action: Box<dyn FnMut(AppHandle, String) + Send + 'static>,
    ) {
        self.commands.insert(
            name.clone(),
            CommandItem {
                name,
                action: Arc::new(Mutex::new(action)),
            },
        );
    }
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
    pub workspace: WorkSpace,
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
                                },
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
        Err("Failed somehow".to_string())
    }
}

pub type AppState = AppStateInner;
