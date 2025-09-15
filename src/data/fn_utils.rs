use freya::prelude::*;

use crate::data::{
	io_utils::save_file,
	stores::{CURRENT_TAB, FILES_ARENA, TABS, close_tab, cycle_tab, delete_tab, new_tab, toggle_command_palette},
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
			log::debug!("CTRL + S was Pressed.");
			let current_tab_content = FILES_ARENA()
				.get(TABS().get(CURRENT_TAB().unwrap()).unwrap().file_key)
				.unwrap()
				.clone();
			save_file(current_tab_content).await;
		}
		Key::Character(c) if (c == "D" && modifiers.contains(Modifiers::SHIFT)) => {
			e.stop_propagation();
			log::debug!("CTRL + SHIFT + D was Pressed.");
			if let Some(tab_index) = CURRENT_TAB() {
				delete_tab(tab_index).await
			}
		}
		Key::Character(c) if c == "w" => {
			e.stop_propagation();
			log::debug!("CTRL + W was Pressed.");
			if let Some(tab_index) = CURRENT_TAB() {
				close_tab(tab_index).await
			}
		}
		Key::Character(c) if c == "t" => {
			e.stop_propagation();
			log::debug!("CTRL + T was Pressed.");
			new_tab().await
		}
		Key::Character(c) if c == "p" => {
			e.stop_propagation();
			log::debug!("CTRL + P was Pressed.");
			toggle_command_palette();
		}
		Key::Tab => {
			e.stop_propagation();
			log::debug!("CTRL + Tab was Pressed.");
			cycle_tab().await;
		}
		_ => (),
	}
}

pub(crate) fn handle_editor_key_input(e: &KeyboardEvent) -> bool {
	let mods = &e.data.modifiers;
	let key = &e.data.key;

	let is_ctrl_char = |ch: &str| mods.contains(Modifiers::CONTROL) && key == &Key::Character(ch.into());

	let skip = is_ctrl_char("s")    // Save
		|| is_ctrl_char("t")        // New tab
		|| is_ctrl_char("p")        // Open command palette
		|| is_ctrl_char("w")        // Close tab
		|| (mods.contains(Modifiers::CONTROL)
		&& mods.contains(Modifiers::SHIFT)
		&& key == &Key::Character("D".into()))  // Delete
		|| (mods.contains(Modifiers::CONTROL)
		&& matches!(e.data.code, Code::Tab)); // Tab cycle

	!skip
}
