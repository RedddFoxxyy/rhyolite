// Copyright (C) 2025  Suyog Tandel(RedddFoxxyy)
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

/*
-------------------------------------------------------------------------
File Index
-------------------------------------------------------------------------
- Imports
- Workspace Store
- Tabs Store
- UI Store
-------------------------------------------------------------------------

-------------------------------------------------------------------------
Devloper Notes:
-------------------------------------------------------------------------
NOTE: I made these global signals because it was the easy way to share a component state between
different components. While this might not be the best way to do it, it works.
TODO: Use Dioxus Radio for complex global states instead of global signals.
-------------------------------------------------------------------------
*/

//! Global Signals and their methods.

//-------------------------------------------------------------------------
// - Imports
//-------------------------------------------------------------------------
use crate::data::{
	io::{delete_file, generate_available_path, get_default_trove_dir, new_file_from_path, save_file},
	themes::ThemesStore,
	types::{DEFAULT_NOTE_TITLE, MarkdownFile, RecentFileInfo, Tab},
};
use dioxus_clipboard::hooks::{UseClipboard, use_clipboard};
use freya::prelude::*;
use slab::Slab;

//-------------------------------------------------------------------------
// - Workspace Store
//-------------------------------------------------------------------------
pub static WORD_CHAR_COUNT: GlobalSignal<(usize, usize)> = Signal::global(|| (0, 0));

// NOTE: Using slab as a memory arena with random file push.
pub static FILES_ARENA: GlobalSignal<Slab<MarkdownFile>> = Signal::global(|| Slab::with_capacity(10));
pub static CURRENT_EDITOR_BUFFER: GlobalSignal<UseEditable> = Signal::global(|| {
	use_editable(
		|| EditableConfig::new("Welcome to Rhyolite!".trim().to_string()).with_allow_tabs(true),
		EditableMode::SingleLineMultipleEditors,
	)
});
pub static ACTIVE_DOCUMENT_TITLE: GlobalSignal<String> = Signal::global(String::new);

pub static RECENT_FILES: GlobalSignal<Vec<RecentFileInfo>> = Signal::global(Vec::new);

pub static PLATFORM: GlobalSignal<UsePlatform> = Signal::global(use_platform);
pub static CLIPBOARD: GlobalSignal<UseClipboard> = Signal::global(use_clipboard);

//-------------------------------------------------------------------------
// - Tabs Store
//-------------------------------------------------------------------------
pub(crate) static TABS: GlobalSignal<Vec<Tab>> = Signal::global(Vec::new);
pub(crate) static CURRENT_TAB: GlobalSignal<Option<usize>> = Signal::global(|| None);

/// Creates a new tab with a new Markdown file.
pub(crate) async fn new_tab() {
	let document_path = generate_available_path(get_default_trove_dir().join(String::from(DEFAULT_NOTE_TITLE) + ".md"));

	let Some(markdownfile) = new_file_from_path(document_path) else {
		log::error!("Failed to create a new tab, due to a previous error!");
		return;
	};

	let file_key = FILES_ARENA.write().insert(markdownfile.clone());
	push_tab(markdownfile.title.clone(), file_key).await;
	// *ACTIVE_DOCUMENT_TITLE.write() = markdownfile.title.clone();
	let log_title = markdownfile.title.clone();

	switch_tab(TABS().len() - 1).await;
	// *CURRENT_TAB.write() = Some(TABS().len() - 1);
	save_file(markdownfile).await;
	log::debug!("Opened New Tab: {log_title}");
}

/// Closes the tab at the given index also freeing its buffer from FILES_BUFFER.
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
		save_file(FILES_ARENA().get(buffer_index).unwrap().clone()).await;
		TABS.write().remove(index);
		FILES_ARENA.write().remove(buffer_index);
		log::debug!("Closed tab: {tab_title}");
	} else {
		log::error!("Failed to close the tab: Invalid tab index! (out of bounds)")
	}
}

/// Closes the tab at the given index also freeing its buffer from FILES_BUFFER, and deletes the file associated with it.
pub async fn delete_tab(index: usize) {
	let Some(tab) = TABS().get(index).cloned() else {
		log::error!("Failed to delete the tab: Invalid tab index! (out of bounds)");
		return;
	};

	let tab_count = TABS().len();
	let current_tab_index = CURRENT_TAB();

	if (tab_count - 1) == 0 {
		new_tab().await;
	}

	TABS.write().remove(index);
	delete_file(FILES_ARENA().get(tab.file_key).unwrap().clone()).await;
	FILES_ARENA.write().remove(tab.file_key);

	match current_tab_index {
		// Deleting the current tab; switch to the previous or stay at 0
		Some(current) if current == index => {
			let new_index = if index > 0 { index - 1 } else { 0 };
			switch_tab(new_index).await;
		}
		// Deleting previous to the current tab; Current tab index needs adjustment due to removal
		Some(current) if current > index => {
			switch_tab(current - 1).await;
		}
		_ => {}
	}

	log::debug!("Closed tab: {}", tab.title);
}

/// Appends a tab of the given title and document index to the TABS vec.
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
		let current_tab_content = FILES_ARENA()
			.get(TABS().get(CURRENT_TAB().unwrap()).unwrap().file_key)
			.unwrap()
			.editable;

		*CURRENT_EDITOR_BUFFER.write() = current_tab_content;
		log::debug!("Switched to tab: {}", tab.title.clone());
	} else {
		log::error!("Failed to switch to the tab: Invalid tab index! (out of bounds)")
	}
}

pub async fn cycle_tab() {
	if let Some(index) = CURRENT_TAB() {
		let total_tabs = TABS().len();
		if total_tabs > (index + 1) {
			switch_tab(index + 1).await;
		} else if total_tabs == (index + 1) {
			switch_tab(0).await;
		}
	} else {
		log::error!("Failed to cycle through the tabs: Invalid Current Tab Value!")
	}
}

//-------------------------------------------------------------------------
// - UI Store
//-------------------------------------------------------------------------

// Fonts:
pub static JET_BRAINS_MONO: &[u8] = include_bytes!("../static/fonts/JetBrainsMono[wght].ttf");

// Stores the current App Theme
pub static THEME_STORE: GlobalSignal<ThemesStore> = Signal::global(ThemesStore::init);

// Sidebar Store:
pub static SHOW_SETTINGS_DROPUP: GlobalSignal<bool> = Signal::global(|| false);
pub static SHOW_THEMES_DROPUP: GlobalSignal<bool> = Signal::global(|| false);
pub static SHOW_COMMAND_PALETTE: GlobalSignal<bool> = Signal::global(|| false);
pub static SHOW_RECENT_FILES: GlobalSignal<bool> = Signal::global(|| false);

// Sidebar Store Methods:
pub fn toggle_settings_dropup() {
	let current_state = SHOW_SETTINGS_DROPUP();
	if current_state && SHOW_THEMES_DROPUP() {
		*SHOW_THEMES_DROPUP.write() = !current_state;
	}
	*SHOW_SETTINGS_DROPUP.write() = !current_state;
}

pub fn toggle_themes_dropup() {
	let current = *SHOW_THEMES_DROPUP.read();
	*SHOW_THEMES_DROPUP.write() = !current;
}

pub fn close_settings_dropup() {
	*SHOW_SETTINGS_DROPUP.write() = false;
	*SHOW_THEMES_DROPUP.write() = false;
}

pub fn toggle_command_palette() {
	let current = *SHOW_COMMAND_PALETTE.read();
	*SHOW_COMMAND_PALETTE.write() = !current;
}

pub fn toggle_recent_files() {
	let current = *SHOW_RECENT_FILES.read();
	*SHOW_RECENT_FILES.write() = !current;
}
