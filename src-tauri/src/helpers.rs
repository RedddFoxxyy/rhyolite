//! ## Module to store Helper Functions(Functions that encloses code that is to be reused
//! ## a lot of times in the code).
//!
//! As of now, this module contains functions that return variables stored in the
//! App State Struct, for example the tab_switcher which is locked behind an RWLock.
//! Functions like `get_tab_switcher()` or `get_tab_switcher_mut()` return a rwlockGuard
//! to the tab_switcher and also handle the errors if it cannot acquire a read/write
//! lock on it.
//!
//! These functions allow you to easily access the app_state variables
//! without having to deal with handling panics/errors and they do not cause any
//! deadlock if they fail!
//!
//! > NOTE: The module is unfinished as of now and more helper functions can be added, or the
//! > existing ones will be modified as required.

#![allow(dead_code)]

use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};

use indexmap::IndexMap;
use uuid::Uuid;

use crate::{
	AppStateInner, // app_state::{CommandRegistry, FileManager, TabManager},
	app_state::{
		CommandRegistry, DEFAULT_NOTE_TITLE, DocumentContent, FileInfo, FileManager, TROVE_DIR,
		Tab, TabManager, USER_DATA_DIR, USER_DATA_FILE, UserData,
	},
	editor::{
		io::{fetch_document_from_disk, get_documents_dir, get_trove_dir},
		settings::themes::Theme,
	},
};
use tauri::async_runtime::{Mutex, RwLock};

impl AppStateInner {
	// Loads all the last session contents from userdata.json and initialises AppStateInner.
	pub fn init_appstate() -> Result<Self, String> {
		log::debug!("Initialising App State.");
		let appdata_dir = get_documents_dir().join(USER_DATA_DIR);
		let userdata_path = appdata_dir.join(USER_DATA_FILE);

		if !userdata_path.exists() {
			// If userdata.json doesn't exist, load all markdown files from the trove directory
			return Self::load_from_default_trove();
		}

		Self::load_from_userdata(&userdata_path)
	}

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
				contents: tab_content.content,
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
				contents: tab_content.content,
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
}
