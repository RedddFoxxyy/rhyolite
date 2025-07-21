use freya::prelude::*;

use crate::data::{
	io_utils::save_file,
	stores::{
		doc_store::FILES_ARENA,
		tabs_store::{CURRENT_TAB, TABS, delete_tab},
	},
};

// #[derive(PartialEq)]
// pub(crate) enum KeyboardInputComponent {
// 	Global,
// 	Editor(UseEditable),
// }

pub(crate) async fn handle_global_keyboard_input(e: KeyboardEvent) {
	let key = &e.data.key;
	let modifiers = e.data.modifiers;

	if !modifiers.contains(Modifiers::CONTROL) {
		return;
	}
	match key {
		Key::Character(c) if c == "s" => {
			e.stop_propagation();
			let current_tab_content = FILES_ARENA()
				.get(TABS().get(CURRENT_TAB().unwrap()).unwrap().buffer_index)
				.unwrap()
				.clone();
			save_file(current_tab_content).await;
		}
		Key::Character(c) if c == "d" => {
			e.stop_propagation();
			delete_tab(CURRENT_TAB().unwrap()).await;
		}
		_ => (),
	}
}
