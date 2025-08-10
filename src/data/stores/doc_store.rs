//! All global signals go here that store the state of the WorkSpace.
//!
//! I made these global signals because it was the easy way to share a component state between
//! different components. While this might not be the best way to do it, it works.

use dioxus_clipboard::hooks::{UseClipboard, use_clipboard};
use freya::prelude::*;
use slab::Slab;

use crate::data::types::{MarkdownFile, RecentFileInfo};

// TODO: Use Dioxus Radio for Global state Management.

// Document Counts Store:
pub static WORD_CHAR_COUNT: GlobalSignal<(usize, usize)> = Signal::global(|| (0, 0));

// Documents Store:
// NOTE: Using slab as a memory arena with random file push.
pub static FILES_ARENA: GlobalSignal<Slab<MarkdownFile>> = Signal::global(|| Slab::with_capacity(10));
pub static CURRENT_EDITOR_BUFFER: GlobalSignal<UseEditable> = Signal::global(|| {
	use_editable(
		|| EditableConfig::new("Welcome to Rhyolite!".trim().to_string()).with_allow_tabs(true),
		EditableMode::SingleLineMultipleEditors,
	)
});
pub static ACTIVE_DOCUMENT_TITLE: GlobalSignal<String> = Signal::global(String::new);

// IO Store:
pub static RECENT_FILES: GlobalSignal<Vec<RecentFileInfo>> = Signal::global(Vec::new);

// Platform Stores:
pub static PLATFORM: GlobalSignal<UsePlatform> = Signal::global(use_platform);
pub static CLIPBOARD: GlobalSignal<UseClipboard> = Signal::global(use_clipboard);
