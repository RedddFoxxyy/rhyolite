use crate::{
	data::{
		io_utils::{deinitialise_app, initialise_app, save_file},
		stores::{
			doc_store::{CURRENT_EDITOR_BUFFER, FILES_ARENA, WORD_CHAR_COUNT},
			tabs_store::{CURRENT_TAB, TABS, delete_tab},
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

	// TODO: Move this to a seprate file/function to handle various key events.
	let onglobalkeydown = async move |e: KeyboardEvent| {
		let key = &e.data.key;
		let modifiers = e.data.modifiers;

		// Check if the Control key is held down and the 's' key is pressed
		if modifiers.contains(Modifiers::CONTROL) && *key == Key::Character("s".to_string()) {
			// println!("Ctrl + S was pressed!");
			e.prevent_default();
			e.stop_propagation();
			let current_tab_content = FILES_ARENA()
				.get(TABS().get(CURRENT_TAB().unwrap()).unwrap().buffer_index)
				.unwrap()
				.clone();
			save_file(current_tab_content).await;
		} else if modifiers.contains(Modifiers::CONTROL) && *key == Key::Character("d".to_string())
		{
			// println!("Ctrl + d was pressed!");
			e.prevent_default();
			e.stop_propagation();
			delete_tab(CURRENT_TAB().unwrap()).await;
		}
	};

	rsx!(rect {
		width: "fill",
		height: "fill",
		background: theme.crust,
		direction: "vertical",
		onglobalkeydown,

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
