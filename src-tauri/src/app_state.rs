//! # App State.
//! Stores the current state and defines core skeleton of the app.
//!
//! All the required global statics are declared in this module.

use std::sync::Arc;
use std::{
	collections::HashMap,
	future::Future,
	path::PathBuf,
	pin::Pin,
};

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, async_runtime::{Mutex, RwLock}};

use crate::editor::settings::themes::Theme;

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
pub const USER_DATA_FILE: &str = "userdata.json";

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
	pub recent_files: Vec<FileInfo>,                      // Stores the list of recently created files
	pub current_theme: Theme,                             // Stores the current theme
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

pub type AppState = AppStateInner;

pub type CommandAction = Box<
	dyn FnMut(AppHandle, Option<String>) -> Pin<Box<dyn Future<Output = ()> + Send>>
		+ Send
		+ Sync
		+ 'static,
>;
