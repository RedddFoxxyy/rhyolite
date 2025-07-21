use crate::{
	data::{
		fn_utils::handle_global_keyboard_input,
		io_utils::{deinitialise_app, initialise_app},
		stores::{
			doc_store::{CURRENT_EDITOR_BUFFER, FILES_ARENA, WORD_CHAR_COUNT},
			tabs_store::{CURRENT_TAB, TABS},
			ui_store::THEME_STORE,
		},
	},
	view::{docview::work_space, sidebar::side_bar, top_bar::top_nav_bar},
};
use freya::prelude::*;

// The initial View for the app, all the app components are a part of this.
pub fn app() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	use_hook(move || {
		initialise_app();
	});

	// Change the current editor on Tab change.
	// NOTE: We can hardcode this logic in the switch tab function itself instead of using a use effect!
	use_effect(move || {
		let current_tab_content = FILES_ARENA()
			.get(TABS().get(CURRENT_TAB().unwrap()).unwrap().buffer_index)
			.unwrap()
			.editable
			.clone();

		*CURRENT_EDITOR_BUFFER.write() = current_tab_content
	});

	// Update the word and char counts on tab change/keyboard input.
	use_effect(move || {
		let editor_content = CURRENT_EDITOR_BUFFER().editor().to_string();
		let char_count = editor_content.chars().count();
		let word_count = editor_content.split_whitespace().count();

		*WORD_CHAR_COUNT.write() = (word_count, char_count);
	});

	use_drop(move || {
		deinitialise_app();
	});

	rsx!(rect {
		width: "fill",
		height: "fill",
		background: theme.crust,
		direction: "vertical",
		onglobalkeydown: handle_global_keyboard_input,

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
