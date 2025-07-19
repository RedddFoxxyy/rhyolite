use crate::data::io_utils::initialise_app;
use crate::data::stores::docspace::{EDITOR_BUFFER, FILES_ARENA};
use crate::data::stores::tabs::{CURRENT_TAB, TABS};
use crate::data::stores::ui::THEME_STORE;
use crate::view::{docview::work_space, sidebar::side_bar, top_bar::top_nav_bar};
use freya::prelude::*;

// The initial View for the app, all the app components are a part of this.
pub fn app() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	use_hook(move || {
		initialise_app();
	});

	// TODO: Instead of changing the content of the editor, spawn an editor per tab and switch it.
	use_effect(move || {
		let current_tab_content = FILES_ARENA()
			.get(TABS().get(CURRENT_TAB().unwrap()).unwrap().buffer_index)
			.unwrap()
			.editable
			.clone();

		*EDITOR_BUFFER.write() = current_tab_content
	});

	rsx!(rect {
		width: "fill",
		height: "fill",
		background: theme.crust,
		direction: "vertical",

		// Tabs Navigation Bar
		top_nav_bar {}

		// Main Workspace
		rect {
			width: "100%",
			height: "fill",
			direction: "horizontal",
			side_bar{},
			work_space{}
		}
	})
}
