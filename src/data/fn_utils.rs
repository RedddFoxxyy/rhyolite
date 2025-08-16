use freya::prelude::*;

use crate::data::{
	io_utils::save_file,
	stores::{
		doc_store::FILES_ARENA,
		tabs_store::{CURRENT_TAB, TABS, close_tab, cycle_tab, delete_tab, new_tab},
		ui_store::toggle_command_palette,
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
			log::info!("CTRL + S was Pressed.");
			let current_tab_content = FILES_ARENA()
				.get(TABS().get(CURRENT_TAB().unwrap()).unwrap().file_key)
				.unwrap()
				.clone();
			save_file(current_tab_content).await;
		}
		Key::Character(c) if (c == "D" && modifiers.contains(Modifiers::SHIFT)) => {
			e.stop_propagation();
			log::info!("CTRL + SHIFT + D was Pressed.");
			if let Some(tab_index) = CURRENT_TAB() {
				delete_tab(tab_index).await
			}
		}
		Key::Character(c) if c == "w" => {
			e.stop_propagation();
			log::info!("CTRL + W was Pressed.");
			if let Some(tab_index) = CURRENT_TAB() {
				close_tab(tab_index).await
			}
		}
		Key::Character(c) if c == "t" => {
			e.stop_propagation();
			log::info!("CTRL + T was Pressed.");
			new_tab().await
		}
		Key::Character(c) if c == "p" => {
			e.stop_propagation();
			log::info!("CTRL + P was Pressed.");
			toggle_command_palette();
		}
		Key::Tab => {
			e.stop_propagation();
			log::info!("CTRL + Tab was Pressed.");
			cycle_tab().await;
		}
		_ => (),
	}
}

pub(crate) fn handle_editor_key_input(e: &KeyboardEvent) -> bool {
	// TODO: Improve the logic and code here.
	let is_save = e.data.modifiers.contains(Modifiers::CONTROL) && e.data.key == Key::Character("s".into());
	let is_new_tab = e.data.modifiers.contains(Modifiers::CONTROL) && e.data.key == Key::Character("t".into());
	let is_open_command_palette = e.data.modifiers.contains(Modifiers::CONTROL) && e.data.key == Key::Character("p".into());
	let is_close_tab = e.data.modifiers.contains(Modifiers::CONTROL) && e.data.key == Key::Character("w".into());

	let is_delete = e.data.modifiers.contains(Modifiers::CONTROL)
		&& e.data.modifiers.contains(Modifiers::SHIFT)
		&& e.data.key == Key::Character("D".into());

	let is_tab_cycle = e.data.modifiers.contains(Modifiers::CONTROL) && matches!(e.data.code, Code::Tab);

	!is_save && !is_delete && !is_tab_cycle && !is_close_tab && !is_open_command_palette && !is_new_tab
}
