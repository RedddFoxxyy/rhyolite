//! All global signals go here that store the state of the Tabs.
//!
//! I made these global signals cause it was the easy way to share a component state between
//! different components. While this might not be the best way to do it, it works.

// NOTE: file, doc, and document all are the same things, MarkdownFileData.

use crate::data::{
	docspace::DOCUMENT_DATA,
	io_utils::{load_default_trove, new_doc_from_path, open_doc_from_path},
	types::Tab,
};
use freya::prelude::{GlobalSignal, Readable, Signal};
// use indexmap::IndexMap;
// use uuid::Uuid;

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

	let new_tab_index = if let Some(index) = CURRENT_TAB() {
		index + 1
	} else {
		0
	};

	// TODO: Handle None
	let doc = new_doc_from_path(document_path);

	let newtab = Tab {
		// index: new_tab_index,
		title: DEFAULT_NOTE_TITLE.to_string(),
		document_index: DOCUMENT_DATA().len(),
	};

	TABS.write().insert(new_tab_index, newtab);
	DOCUMENT_DATA.write().push(doc);
	*CURRENT_TAB.write() = Some(new_tab_index);
}

fn add_tab(title: String, file_index: usize) {
	let newtab = Tab {
		// index: new_tab_index,
		title,
		document_index: file_index,
	};

	TABS.write().push(newtab);
}

/// Loads last open tabs or all tabs in default trove.
pub fn initialise_app() {
	let markdownfiles = load_default_trove();

	if markdownfiles.is_empty() {
		new_tab();
	} else {
		for file in markdownfiles {
			let insertion_index = DOCUMENT_DATA().len();
			add_tab(file.title.clone(), insertion_index);
			DOCUMENT_DATA.write().push(file);
		}
	}
}
