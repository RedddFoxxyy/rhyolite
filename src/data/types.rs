//! # App State.
//! Stores the current state and defines core skeleton of the app.
//!
//! All the required global statics/constants are declared in this module.

use crate::data::themes::Theme;
use freya::hooks::UseEditable;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Name of the Default Note Title used by the app!
pub const APP_DATA_DIR: &str = "Rhyolite";

/// Name of the Default Note Title used by the app!
pub const USER_DATA_DIR: &str = "appdata";

pub const USER_DATA_FILE: &str = "userdata.toml";

/// Name of the Default Trove used by the app!
pub const DEFAULT_TROVE_DIR: &str = "Untitled_Trove";

/// Name of the Default Note Title used by the app!
pub const DEFAULT_NOTE_TITLE: &str = "Untitled";

#[derive(Clone, PartialEq)]
pub struct MarkdownFile {
	pub path: PathBuf,
	pub title: String,
	pub editable: UseEditable,
}

/// Denotes a tab in the editor.
#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Tab {
	// pub index: usize,  // Unique identifier for the tab ( removed it for now )
	pub title: String, // Title of the Document
	pub file_path: PathBuf,
	pub file_key: usize, // The reference to the document in the document vec.
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct OpenFileData {
	pub title: String,
	pub contents: String,
}

///Userdata Struct, used to store the userdata, like last open tab and all the open tabs.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UserData {
	pub active_tabs: Vec<Tab>, // Stores the list of last active tabs before the editor was closed
	pub last_open_tab: usize,  // Stores the tab id of the last open tab
	pub recent_files: Vec<RecentFileInfo>, // Stores the list of recently created files
	pub current_theme: Theme,  // Stores the current theme color palette
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecentFileInfo {
	pub id: String,
	pub title: String,
	pub path: PathBuf,
}
