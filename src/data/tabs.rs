//! All global signals go here that store the state of the Tabs.
//!
//! I made these global signals cause it was the easy way to share a component state between
//! different components. While this might not be the best way to do it, it works.

use crate::data::types::Tab;
use freya::prelude::{GlobalSignal, Readable, Signal};
// use indexmap::IndexMap;
use uuid::Uuid;

use super::{
	io_utils::{generate_available_path, get_trove_dir},
	types::{DEFAULT_NOTE_TITLE, DEFAULT_TROVE_DIR, MarkdownFileData},
};

// Tabs Store:
pub static TABS: GlobalSignal<Vec<Tab>> = Signal::global(Vec::new);
pub static CURRENT_TAB: GlobalSignal<Option<usize>> = Signal::global(|| None);

// Tab Methods:

/// Creates a new tab with a new markdown file.
fn new_tab() {
	let document_path =
		generate_available_path(get_trove_dir(DEFAULT_TROVE_DIR).join(DEFAULT_NOTE_TITLE));

	// TODO: Handle None
	let current_tab_index = CURRENT_TAB().unwrap();
	let new_tab_index = current_tab_index + 1;

	let newtab = Tab {
		index: new_tab_index,
		title: DEFAULT_NOTE_TITLE.to_string(),
		document: Box::new(Some(MarkdownFileData {
			title: DEFAULT_NOTE_TITLE.to_string(),
			content: String::new(),
			path: document_path,
		})),
	};
	TABS().insert(new_tab_index, newtab);
	*CURRENT_TAB.write() = Some(new_tab_index);
}
