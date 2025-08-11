//! All global signals go here that store the state of the Tabs.
//!
//! I made these global signals because it was the easy way to share a component state between
//! different components. While this might not be the best way to do it, it works.

// NOTE: file, doc, and document all are the same things, MarkdownFileData.

use crate::data::io_utils::{delete_file, generate_available_path, get_trove_dir, save_file};
use crate::data::stores::doc_store::ACTIVE_DOCUMENT_TITLE;
use crate::data::types::{DEFAULT_NOTE_TITLE, DEFAULT_TROVE_DIR};
use crate::data::{io_utils::new_file_from_path, stores::doc_store::FILES_ARENA, types::Tab};
use freya::prelude::{GlobalSignal, Signal};

// Tabs Store:
pub(crate) static TABS: GlobalSignal<Vec<Tab>> = Signal::global(Vec::new);
pub(crate) static CURRENT_TAB: GlobalSignal<Option<usize>> = Signal::global(|| None);

// Tab Methods:

/// Creates a new tab with a new markdown file.
pub(crate) async fn new_tab() {
	let document_path = generate_available_path(get_trove_dir(DEFAULT_TROVE_DIR).join(String::from(DEFAULT_NOTE_TITLE) + ".md"));

	let Some(markdownfile) = new_file_from_path(document_path) else {
		log::error!("Failed to create a new tab, due to a previous error!");
		return;
	};

	let file_key = FILES_ARENA.write().insert(markdownfile.clone());
	push_tab(markdownfile.title.clone(), file_key).await;
	*ACTIVE_DOCUMENT_TITLE.write() = markdownfile.title.clone();
	let log_title = markdownfile.title.clone();

	*CURRENT_TAB.write() = Some(TABS().len() - 1);
	save_file(markdownfile).await;
	log::info!("Opened New Tab: {log_title}");
}

/// Closes the tab at given index also freeing its buffer from FILES_BUFFER.
pub async fn close_tab(index: usize) {
	if let Some(tab) = TABS().get(index) {
		let tab_title = tab.title.clone();
		let buffer_index = tab.file_key;
		let tab_count = TABS().len();
		if CURRENT_TAB() == Some(index) {
			if index != 0 {
				switch_tab(index - 1).await;
			} else if tab_count > 1 {
				switch_tab(index + 1).await;
			} else {
				return;
			}
		}
		// TODO: Trigger a file save before closing the tab.
		save_file(FILES_ARENA().get(buffer_index).unwrap().clone()).await;
		TABS.write().remove(index);
		FILES_ARENA.write().remove(buffer_index);
		log::info!("Closed tab: {tab_title}");
	} else {
		log::error!("Failed to close the tab: Invalid tab index! (out of bounds)")
	}
}

/// Closes the tab at given index also freeing its buffer from FILES_BUFFER, and deletes the file associated with it.
pub async fn delete_tab(index: usize) {
	if let Some(tab) = TABS().get(index) {
		let tab_title = tab.title.clone();
		let buffer_index = tab.file_key;
		let tab_count = TABS().len();
		if CURRENT_TAB() == Some(index) {
			if index != 0 {
				switch_tab(index - 1).await;
			} else if tab_count > 1 {
				switch_tab(index + 1).await;
			} else {
				return;
			}
		}
		// TODO: Trigger a file save before closing the tab.
		TABS.write().remove(index);
		delete_file(FILES_ARENA().get(buffer_index).unwrap().clone()).await;
		FILES_ARENA.write().remove(buffer_index);
		log::info!("Deleted file: {tab_title}");
	} else {
		log::error!("Failed to delete the tab: Invalid tab index! (out of bounds)")
	}
}

/// Appends a tab of given title and document index to the TABS vec.
pub async fn push_tab(title: String, file_key: usize) {
	let file_path = FILES_ARENA().get(file_key).unwrap().path.clone();
	let newtab = Tab {
		title,
		file_path,
		file_key,
	};
	TABS.write().push(newtab);
}

pub async fn switch_tab(index: usize) {
	if let Some(tab) = TABS().get(index) {
		*CURRENT_TAB.write() = Some(index);
		*ACTIVE_DOCUMENT_TITLE.write() = tab.title.clone();
		log::info!("Swiched to tab: {}", tab.title.clone());
	} else {
		log::error!("Failed to switch to the tab: Invalid tab index! (out of bounds)")
	}
}
