//! # App State.
//! Stores the current state and defines core skeleton of the app.
//!
//! All the required global statics/constants are declared in this module.

use freya::prelude::Signal;
use std::sync::Arc;
use std::{collections::HashMap, path::PathBuf};

// use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::data::themes::Theme;

/// Name of the Default Note Title used by the app!
pub const APP_DATA_DIR: &str = "Rhyolite";

/// Name of the Default Note Title used by the app!
pub const USER_DATA_DIR: &str = "appdata";

pub const USER_DATA_FILE: &str = "userdata.toml";

/// Name of the Default Trove used by the app!
pub const DEFAULT_TROVE_DIR: &str = "Untitled_Trove";

/// Name of the Default Note Title used by the app!
pub const DEFAULT_NOTE_TITLE: &str = "Untitled";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarkdownFile {
	pub path: PathBuf,
	pub title: String,
	pub content: String,
}

/// Denotes a tab in the editor.
/// Has a unique identifier and a title(where title is the title of the Markdown File).
#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Tab {
	// pub index: usize,  // Unique identifier for the tab ( removed it for now )
	pub title: String,       // Title of the Document
	pub buffer_index: usize, // The reference to the document in the document vec.
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Buffer {
	pub title: String,
	pub contents: String,
}

///Userdata Struct, used to store the userdata, like last open tab and all the open tabs.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserData {
	pub active_tabs: Vec<Tab>, // Stores the list of last active tabs before the editor was closed
	pub last_open_tab: String, // Stores the tab id of the last open tab
	pub recent_files: Vec<FileInfo>, // Stores the list of recently created files
	pub current_theme: Theme,  // Stores the current theme color palette
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileInfo {
	pub id: String,
	pub title: String,
	pub path: PathBuf,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FileManager {
	pub documents: HashMap<String, Arc<Buffer>>, // Used to store open documents in the editor (tabid, tabdocument)
	pub recent_files: Vec<FileInfo>,             // Stores the list of recently created files
}
impl Default for FileManager {
	fn default() -> Self {
		Self {
			documents: HashMap::new(),
			recent_files: Vec::new(),
		}
	}
}
