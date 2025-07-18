//! All global signals go here that store the state of the Tabs.
//!
//! I made these global signals cause it was the easy way to share a component state between
//! different components. While this might not be the best way to do it, it works.

// NOTE: file, doc, and document all are the same things, MarkdownFileData.

use crate::data::io_utils::{generate_available_path, get_trove_dir};
use crate::data::stores::docspace::ACTIVE_DOCUMENT_TITLE;
use crate::data::types::{DEFAULT_NOTE_TITLE, DEFAULT_TROVE_DIR};
use crate::data::{
	io_utils::{load_default_trove, new_file_from_path, open_file_from_path},
	stores::docspace::FILES_BUFFER,
	types::Tab,
};
use freya::prelude::{GlobalSignal, Readable, Signal};

// Tabs Store:
pub(crate) static TABS: GlobalSignal<Vec<Tab>> = Signal::global(Vec::new);
pub(crate) static CURRENT_TAB: GlobalSignal<Option<usize>> = Signal::global(|| None);

// Tab Methods:

/// Creates a new tab with a new markdown file.
pub(crate) fn new_tab() {
	let document_path =
		generate_available_path(get_trove_dir(DEFAULT_TROVE_DIR).join(DEFAULT_NOTE_TITLE));

	let Some(file) = new_file_from_path(document_path) else {
		log::error!("Failed to create a new tab, due to a previous error!");
		return;
	};
	push_tab(file.title.clone(), FILES_BUFFER().len());
	*ACTIVE_DOCUMENT_TITLE.write() = file.title.clone();

	FILES_BUFFER.write().push(file);
	*CURRENT_TAB.write() = Some(TABS().len() - 1);
}

/// Closes the tab at given index also freeing the its buffer from FILES_BUFFER.
pub fn close_tab(index: usize) {
	if let Some(tab) = TABS().get(index) {
		let buffer_index = tab.buffer_index;
		if CURRENT_TAB() == Some(index) {
			switch_tab(index - 1);
		}
		// TODO: Trigger a file save before closing the tab.
		TABS.write().remove(index);
		FILES_BUFFER.write().remove(buffer_index);
	} else {
		log::error!("Failed to switch to the tab: Invalid tab index! (out of bounds)")
	}
}

/// Appends a tab of given title and document index to the TABS vec.
pub(crate) fn push_tab(title: String, document_index: usize) {
	let newtab = Tab {
		title,
		buffer_index: document_index,
	};
	TABS.write().push(newtab);
}

pub(crate) fn switch_tab(index: usize) {
	if let Some(tab) = TABS().get(index) {
		*CURRENT_TAB.write() = Some(index);
		*ACTIVE_DOCUMENT_TITLE.write() = tab.title.clone();
	} else {
		log::error!("Failed to switch to the tab: Invalid tab index! (out of bounds)")
	}
}
