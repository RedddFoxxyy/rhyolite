//! All global signals go here that store the state of the UI.
//!
//! I made these global signals cause it was the easy way to share a component state between
//! different components. While this might not be the best way to do it, it works.

use crate::utils::themes::ThemesStore;
use freya::prelude::{GlobalSignal, Readable, Signal};

// Fonts:
pub static JET_BRAINS_MONO: &[u8] = include_bytes!("../static/fonts/JetBrainsMono[wght].ttf");

// Stores the current App Theme
pub static THEME_STORE: GlobalSignal<ThemesStore> = Signal::global(ThemesStore::default);

// Sidebar Store:
pub static SHOW_SETTINGS_DROPUP: GlobalSignal<bool> = Signal::global(|| false);
pub static SHOW_THEMES_DROPUP: GlobalSignal<bool> = Signal::global(|| false);
pub static SHOW_COMMAND_PALETTE: GlobalSignal<bool> = Signal::global(|| false);
pub static SHOW_RECENT_FILES: GlobalSignal<bool> = Signal::global(|| false);

// Sidebar Store Methods:
pub fn toggle_settings_dropup() {
	let current = *SHOW_SETTINGS_DROPUP.read();
	if current && SHOW_THEMES_DROPUP() {
		*SHOW_THEMES_DROPUP.write() = !current;
	}
	*SHOW_SETTINGS_DROPUP.write() = !current;
}
pub fn toggle_themes_dropup() {
	let current = *SHOW_THEMES_DROPUP.read();
	*SHOW_THEMES_DROPUP.write() = !current;
}
pub fn toggle_command_palette() {
	let current = *SHOW_COMMAND_PALETTE.read();
	*SHOW_COMMAND_PALETTE.write() = !current;
}
pub fn toggle_recent_files() {
	let current = *SHOW_RECENT_FILES.read();
	*SHOW_RECENT_FILES.write() = !current;
}
