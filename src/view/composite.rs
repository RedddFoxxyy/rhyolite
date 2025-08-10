#[allow(unused_imports)]
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
use winit::window::ResizeDirection;

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
			.get(TABS().get(CURRENT_TAB().unwrap()).unwrap().file_key)
			.unwrap()
			.editable;

		*CURRENT_EDITOR_BUFFER.write() = current_tab_content
	});

	// Update the word and char counts on tab change/keyboard input.
	use_effect(move || {
		let editor_content = CURRENT_EDITOR_BUFFER().editor().to_string();
		let char_count = editor_content.chars().count();
		let word_count = editor_content.split_whitespace().count();

		*WORD_CHAR_COUNT.write() = (word_count, char_count);
	});

	// NOTE: Do not run this here, I am still figuring out how to deinitialise the app correctly,
	// for now I am running this funciton in docview/document_editor.
	// use_drop(move || {
	// 	deinitialise_app();
	// });

	rsx!(rect {
		width: "fill",
		height: "fill",
		background: theme.crust,
		direction: "vertical",
		onglobalkeydown: handle_global_keyboard_input,

		drag_resize_area {}

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

#[component]
fn drag_resize_area() -> Element {
	let platform = use_platform();
	// NOTE: Adjust this value for resizing handles.
	const BORDER_SIZE: u8 = 8;

	let create_resize_handler = |direction: ResizeDirection| {
		move |_| {
			platform.with_window(move |window| {
				let _ = window.drag_resize_window(direction);
			});
		}
	};

	let create_set_cursor_handler = |cursor: CursorIcon| {
		move |_| {
			platform.set_cursor(cursor);
		}
	};

	let reset_cursor_handler = move |_| {
		platform.set_cursor(CursorIcon::Default);
	};

	rsx! {
		// window corners
		// Top-Left
		rect {
			position: "absolute", layer: "overlay",
			position_top: "0", position_left: "0",
			width: "{BORDER_SIZE}", height: "{BORDER_SIZE}",
			onmousedown: create_resize_handler(ResizeDirection::NorthWest),
			onmouseenter: create_set_cursor_handler(CursorIcon::NwResize),
			onmouseleave: reset_cursor_handler,
		}
		// Top-Right
		rect {
			position: "absolute", layer: "overlay",
			position_top: "0", position_right: "0",
			width: "{BORDER_SIZE}", height: "{BORDER_SIZE}",
			onmousedown: create_resize_handler(ResizeDirection::NorthEast),
			onmouseenter: create_set_cursor_handler(CursorIcon::NeResize),
			onmouseleave: reset_cursor_handler,
		}
		// Bottom-Left
		rect {
			position: "absolute", layer: "overlay",
			position_bottom: "0", position_left: "0",
			width: "{BORDER_SIZE}", height: "{BORDER_SIZE}",
			onmousedown: create_resize_handler(ResizeDirection::SouthWest),
			onmouseenter: create_set_cursor_handler(CursorIcon::SwResize),
			onmouseleave: reset_cursor_handler,
		}
		// Bottom-Right
		rect {
			position: "absolute", layer: "overlay",
			position_bottom: "0", position_right: "0",
			width: "{BORDER_SIZE}", height: "{BORDER_SIZE}",
			onmousedown: create_resize_handler(ResizeDirection::SouthEast),
			onmouseenter: create_set_cursor_handler(CursorIcon::SeResize),
			onmouseleave: reset_cursor_handler,
		}

		// Window edges
		// Top
		rect {
			position: "absolute", layer: "overlay",
			position_top: "0", position_left: "{BORDER_SIZE}",
			height: "{BORDER_SIZE}",
			width: "calc(100% - {2 * BORDER_SIZE}px)",
			onmousedown: create_resize_handler(ResizeDirection::North),
			onmouseenter: create_set_cursor_handler(CursorIcon::NResize),
			onmouseleave: reset_cursor_handler,
		}
		// Bottom
		rect {
			position: "absolute", layer: "overlay",
			position_bottom: "0", position_left: "{BORDER_SIZE}",
			height: "{BORDER_SIZE}",
			width: "calc(100% - {2 * BORDER_SIZE}px)",
			onmousedown: create_resize_handler(ResizeDirection::South),
			onmouseenter: create_set_cursor_handler(CursorIcon::SResize),
			onmouseleave: reset_cursor_handler,
		}
		// Left
		rect {
			position: "absolute", layer: "overlay",
			position_top: "{BORDER_SIZE}", position_left: "0",
			width: "{BORDER_SIZE}",
			height: "calc(100% - {2 * BORDER_SIZE}px)",
			onmousedown: create_resize_handler(ResizeDirection::West),
			onmouseenter: create_set_cursor_handler(CursorIcon::WResize),
			onmouseleave: reset_cursor_handler,
		}
		// Right
		rect {
			position: "absolute", layer: "overlay",
			position_top: "{BORDER_SIZE}", position_right: "0",
			width: "{BORDER_SIZE}",
			height: "calc(100% - {2 * BORDER_SIZE}px)",
			onmousedown: create_resize_handler(ResizeDirection::East),
			onmouseenter: create_set_cursor_handler(CursorIcon::EResize),
			onmouseleave: reset_cursor_handler,
		}
	}
}
