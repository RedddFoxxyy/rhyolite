//! # App State.
//! Stores the current state and defines core skeleton of the app.
//!
//! All the required global statics/constants are declared in this module.

use freya::prelude::Signal;
use std::sync::Arc;
use std::{collections::HashMap, future::Future, path::PathBuf, pin::Pin};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::utils::themes::Theme;

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
pub const USER_DATA_FILE: &str = "userdata.toml";

// TODO: If you find any code in the code base, that uses
// string "Untitled_Trove" instead of this constant, replace it with
// this constant!
/// Name of the Default Trove used by the app!
pub const DEFAULT_TROVE_DIR: &str = "Untitled_Trove";

// TODO: If you find any code in the code base, that uses
// string "Untitled" instead of this constant, replace it with
// this constant!
/// Name of the Default Note Title used by the app!
pub const DEFAULT_NOTE_TITLE: &str = "Untitled";

/// Not to be confused with Document struct, this struct denotes a markdown file.
/// It stores the id( a unique document identifier ), title and path of the markdown file.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarkdownFileData {
	pub path: PathBuf,
	pub title: String,
	pub content: String,
}

/// Denotes a tab in the editor.
/// Has a unique identifier and a title(where title is the title of the Markdown File).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tab {
	pub id: Uuid,      // Unique identifier for the tab (usually document Path)
	pub title: String, // Title of the Document
	pub document: Box<Option<MarkdownFileData>>,
}

///Userdata Struct, used to store the userdata, like last open tab and all the open tabs.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData {
	pub active_tabs: Vec<Tab>, // Stores the list of last active tabs before the editor was closed
	pub last_open_tab: String, // Stores the tab id of the last open tab
	pub recent_files: Vec<FileInfo>, // Stores the list of recently created files
	pub current_theme: Theme,  // Stores the current theme color palette
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
#[derive(Debug)]
pub struct FileManager {
	pub documents: HashMap<String, Arc<DocumentContent>>, // Used to store open documents in the editor (tabid, tabdocument)
	pub recent_files: Vec<FileInfo>,                      // Stores the list of recently created files
}
impl Default for FileManager {
	fn default() -> Self {
		Self {
			documents: HashMap::new(),
			recent_files: Vec::new(),
		}
	}
}
