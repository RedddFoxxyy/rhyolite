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
				.get(TABS().get(CURRENT_TAB().unwrap()).unwrap().file_key)
				.unwrap()
				.clone();
			save_file(current_tab_content).await;
		}
		Key::Character(c) if (c == "D" && modifiers.contains(Modifiers::SHIFT)) => {
			e.stop_propagation();
			delete_tab(CURRENT_TAB().unwrap()).await;
		}
		_ => (),
	}
}

pub(crate) fn handle_editor_key_input(e: &KeyboardEvent) -> bool {
	// TODO: Improve the logic and code here.
	let is_save =
		e.data.modifiers.contains(Modifiers::CONTROL) && e.data.key == Key::Character("s".into());

	let is_delete = e.data.modifiers.contains(Modifiers::CONTROL)
		&& e.data.modifiers.contains(Modifiers::SHIFT)
		&& e.data.key == Key::Character("D".into());

	return !is_save && !is_delete;
}
