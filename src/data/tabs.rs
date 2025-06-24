//! All global signals go here that store the state of the Tabs.
//!
//! I made these global signals cause it was the easy way to share a component state between
//! different components. While this might not be the best way to do it, it works.

use crate::data::types::Tab;
use freya::prelude::{GlobalSignal, Readable, Signal};
use indexmap::IndexMap;
use uuid::Uuid;

use super::{
	io_utils::{generate_available_path, get_trove_dir},
	types::{DEFAULT_NOTE_TITLE, DEFAULT_TROVE_DIR, MarkdownFileData},
};

// Tabs Store:
pub static TABS: GlobalSignal<IndexMap<Uuid, Tab>> = Signal::global(IndexMap::new);
pub static CURRENT_TAB: GlobalSignal<Option<Uuid>> = Signal::global(|| None);

// Tab Methods:

/// Creates a new tab with a new markdown file.
fn new_tab() {
	let document_path =
		generate_available_path(get_trove_dir(DEFAULT_TROVE_DIR).join(DEFAULT_NOTE_TITLE));
	let tab_id = Uuid::new_v4();
	let newtab = Tab {
		id: tab_id,
		title: DEFAULT_NOTE_TITLE.to_string(),
		document: Box::new(Some(MarkdownFileData {
			title: DEFAULT_NOTE_TITLE.to_string(),
			content: String::new(),
			path: document_path,
		})),
	};
	TABS().insert(tab_id, newtab);
	*CURRENT_TAB.write() = Some(tab_id);
}
