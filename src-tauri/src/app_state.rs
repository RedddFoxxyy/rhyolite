//! # App State.
//! Stores the current state and defines core skeleton of the app.
//!
//! All the required global statics are declared in this module.

use std::sync::Arc;
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

use crate::editor::io::fetch_document_from_disk;
use crate::editor::{
	io::{get_documents_dir, get_trove_dir},
	settings::themes::Theme,
};

// TODO: If you find any code in the code base, that uses
// string "Rhyolite" instead of this constant, replace it with
// this constant!
/// Name of the Default Note Title used by the app!
pub const APP_DATA_DIR: &str = "Rhyolite";

// TODO: If you find any code in the code base, that uses
// string "appdata" instead of this constant, replace it with
// this constant!
/// Name of the Default Note Title used by the app!
pub const USER_DATA_DIR: &str = "appdata";

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

/// Not to be confused with Document struct, this struct denotes a markdown file.
/// It stores the id( a unique document identifier ), title and path of the markdown file.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarkdownFileData {
	pub id: String,
	pub title: String,
	pub content: String,
}

/// Denotes a tab in the editor.
/// Has a unique identifier and a title(where title is the title of the Markdown File).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tab {
	pub id: String,    // Unique identifier for the tab
	pub title: String, // Title of the tab
}

///Userdata Struct, used to store the userdata, like last open tab and all the open tabs.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData {
	pub active_tabs: Vec<Tab>, // Stores the list of last active tabs before the editor was closed
	pub last_open_tab: String, // Stores the tab id of the last open tab
	pub recent_files: Vec<FileInfo>, // Stores the list of recently created files
	pub current_theme: Theme,  // Stores the current theme color palette
}

#[derive(Debug, Default, Clone)]
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

/// Document open in a tab. Not to be confused with MarkdownFileData.
///
/// This struct is used to store the contents of a document open in a tab, so
/// that it can be loaded from here instead of storage on tab switch.
///
/// As of now a document can be a markdown file or
/// a graph of connections between markdown files( yet to be implemented ).
#[allow(dead_code)]
#[derive(Debug)]
pub struct DocumentContent {
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
	pub documents: HashMap<String, Arc<DocumentContent>>, // Used to store open documents in the editor (tabid, tabdocument)
	pub recent_files: Vec<FileInfo>,             // Stores the list of recently created files
	pub current_theme: Theme,                    // Stores the current theme
}

#[derive(Default)]
pub struct AppStateInner {
	// Q: Should the TabManager have an rwlock or should the elements in
	// TabManager have RwLock
	// TODO: The elements in TabManager should have RwLock instead of TabManger.
	pub tab_switcher: RwLock<TabManager>,
	pub active_tab_switch: Arc<Mutex<()>>, // Used to avoid tab switching race condition.
	pub command_registry: Mutex<CommandRegistry>,
	pub workspace: RwLock<FileManager>,
}

impl AppStateInner {
	pub fn load_from_userdata(userdata_path: &PathBuf) -> Result<Self, String> {
		// Load the userdata.json content as string.
		let json_content = fs::read_to_string(userdata_path);

		// Handle the case if it fails to load userdata.json content as string.
		if json_content.is_err() {
			// If reading the file fails, log the error
			let error = json_content.unwrap_err();
			log::warn!(
				"Failed to read userdata file: {}. Proceeding with default.",
				error
			);
			return Err(format!("Failed to init app: {}", error));
		}

		// Attempt to Deserialise the json to UserData
		let maybe_user_data = serde_json::from_str::<UserData>(&json_content.unwrap());
		// If deserialization fails, log the error and delete the file
		if maybe_user_data.is_err() {
			let error = maybe_user_data.unwrap_err();
			log::warn!(
				"Failed to deserialize userdata: {}. Deleting the file.",
				error
			);

			// Attempt to delete the problematic userdata file
			if let Err(delete_err) = fs::remove_file(userdata_path) {
				log::error!("Failed to delete corrupted userdata file: {}", delete_err);
			}
			return Err(format!("Failed to init app: {}", error));
		}

		let user_data = maybe_user_data.unwrap();
		let recent_files = user_data.recent_files.clone();
		let current_tab_id = Some(user_data.last_open_tab.clone());
		let current_theme = user_data.current_theme.clone();
		let tabs: IndexMap<String, Tab> = user_data
			.active_tabs
			.iter()
			.map(|d| (d.id.to_string(), d.clone()))
			.collect();
		
			let mut tab_documents: HashMap<String, Arc<DocumentContent>> = HashMap::new();
			for tab in tabs.iter() {
				let tab_data = tab.1.clone();
				let maybe_tab_content = fetch_document_from_disk(tab_data);
				
				if maybe_tab_content.is_none() {
					return Err("Failed to load the documents".to_string());
				}
				let tab_content = maybe_tab_content.unwrap();
				let tab_document = Arc::new(DocumentContent {
					title: tab_content.title,
					contents: tab_content.content
				});
				tab_documents.insert(tab.0.clone(), tab_document);
			}
			
		Ok(Self {
			tab_switcher: RwLock::new(TabManager {
				current_tab_id,
				tabs,
			}),
			workspace: FileManager {
				documents: tab_documents,
				recent_files,
				current_theme,
			}
			.into(),
			..Default::default()
		})
	}

	pub fn load_from_default_trove() -> Result<Self, String> {
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

		// Cache the contents of open tabs in memory
		let mut tab_documents: HashMap<String, Arc<DocumentContent>> = HashMap::new();
		for tab in tabs.iter() {
			let tab_data = tab.1.clone();
			let maybe_tab_content = fetch_document_from_disk(tab_data);
			
			if maybe_tab_content.is_none() {
				return Err("Failed to load the documents".to_string());
			}
			let tab_content = maybe_tab_content.unwrap();
			let tab_document = Arc::new(DocumentContent {
				title: tab_content.title,
				contents: tab_content.content
			});
			tab_documents.insert(tab.0.clone(), tab_document);
		}

		Ok(Self {
			tab_switcher: RwLock::new(TabManager {
				current_tab_id,
				tabs,
			}),
			active_tab_switch: Arc::new(Mutex::new(())),
			workspace: FileManager {
				documents: tab_documents,
				recent_files,
				current_theme: Theme::default(),
			}
			.into(),
			command_registry: Mutex::new(CommandRegistry::default()),
		})
	}

	// Loads all the last session contents from userdata.json and initialises AppStateInner.
	pub fn init_appstate() -> Result<Self, String> {
		log::debug!("Initialising App State.");
		let appdata_dir = get_documents_dir().join("appdata");
		let userdata_path = appdata_dir.join("userdata.json");

		if !userdata_path.exists() {
			// If userdata.json doesn't exist, load all markdown files from the trove directory
			return Self::load_from_default_trove();
		}

		Self::load_from_userdata(&userdata_path)
	}
}

pub type AppState = AppStateInner;

pub type CommandAction = Box<
	dyn FnMut(AppHandle, Option<String>) -> Pin<Box<dyn Future<Output = ()> + Send>>
		+ Send
		+ Sync
		+ 'static,
>;
