#[allow(unused_imports)]
use crate::{
	data::{
		fn_utils::handle_global_keyboard_input,
		io_utils::{deinitialise_app, initialise_app},
		stores::{
			doc_store::{CURRENT_EDITOR_BUFFER, FILES_ARENA, WORD_CHAR_COUNT},
			tabs_store::{CURRENT_TAB, TABS},
			ui_store::{
				SHOW_COMMAND_PALETTE, SHOW_RECENT_FILES, SHOW_SETTINGS_DROPUP, THEME_STORE, close_settings_dropup, toggle_command_palette,
				toggle_recent_files,
			},
		},
	},
	view::{docview::work_space, palette::palette_box, sidebar::side_bar, top_bar::top_nav_bar},
};
use freya::prelude::*;
use winit::window::ResizeDirection;

/// The main view of the app, this is where the first parent components are layed out.
pub fn app() -> Element {
	let theme = THEME_STORE().current_theme.colors;
	const BORDER_SIZE: u8 = 6;

	use_hook(move || {
		initialise_app();
	});

	// Update the word and char counts on tab change/keyboard input.
	use_effect(move || {
		let editor_content = CURRENT_EDITOR_BUFFER().editor().to_string();
		let char_count = editor_content.chars().count();
		let word_count = editor_content.split_whitespace().count();

		*WORD_CHAR_COUNT.write() = (word_count, char_count);
	});

	// NOTE: Do not run this here, I am still figuring out how to deinitialise the app correctly,
	// for now I am running this function in docview/document_editor.
	// use_drop(move || {
	// 	deinitialise_app();
	// });

	rsx!(
		// 1
		rect {
			width: "fill",
			height: "fill",
			direction: "vertical",

			background: theme.crust,
			direction: "vertical",
			onglobalkeydown: handle_global_keyboard_input,
			drag_resize_area {border_size: BORDER_SIZE}

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

			if SHOW_COMMAND_PALETTE() ^ SHOW_RECENT_FILES() ^ SHOW_SETTINGS_DROPUP() {
				overlay_view{}
			}
		}
	)
}

/// Handles the overlay view; this is where onclick handlers for closing the floating components (like palettes and settings dropup are handled).
/// This is a hacky way to implement this; once a stable API for such stuff is implemented in Freya lib, then this will be replaced by those.
///
/// Other ways of doing this are by using `use_focus()` to close the floating windows when unfocussed or by using the `onglobalclick` handler.
#[component]
pub fn overlay_view() -> Element {
	let _focus = use_focus();
	let backdrop_blur_value: u8 = if SHOW_SETTINGS_DROPUP() { 0 } else { 1 };
	let background_color = if SHOW_SETTINGS_DROPUP() {
		"rgb(0, 0, 0, 0.0)"
	} else {
		"rgb(0, 0, 0, 0.2)"
	};

	rsx!(rect {
		position: "global",
		position_top: "0",
		position_left: "0",
		width: "100%",
		height: "100%",
		main_align: "center",
		cross_align: "center",
		background: background_color,
		backdrop_blur: "{backdrop_blur_value}",
		layer: "overlay",

		onclick: move |e| {
			e.stop_propagation();
			if SHOW_RECENT_FILES() {
				toggle_recent_files();
			} else if SHOW_COMMAND_PALETTE() {
				toggle_command_palette();
			} else if SHOW_SETTINGS_DROPUP() {
				close_settings_dropup();
			}
		},

		if SHOW_COMMAND_PALETTE() ^ SHOW_RECENT_FILES() {
			// TODO: Implement the floating palettes.
			palette_box{
				// if SHOW_RECENT_FILES() {
				// 	recent_files_palette{}
				// } else {
				// 	command_palette{}
				// }
			}
		}
	})
}

/// A thin 6-8 pixels border over the app window running around the edges to handle window resizing.
#[component]
fn drag_resize_area(border_size: u8) -> Element {
	let platform = use_platform();
	// NOTE: Adjust this value for resizing handles.
	let platform_information = use_platform_information();
	let is_maximised = platform_information().is_maximized;

	if is_maximised {
		return rsx! {};
	}

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
			width: "{border_size}", height: "{border_size}",
			onmousedown: create_resize_handler(ResizeDirection::NorthWest),
			onmouseenter: create_set_cursor_handler(CursorIcon::NwResize),
			onmouseleave: reset_cursor_handler,
		}
		// Top-Right
		rect {
			position: "absolute", layer: "overlay",
			position_top: "0", position_right: "0",
			width: "{border_size}", height: "{border_size}",
			onmousedown: create_resize_handler(ResizeDirection::NorthEast),
			onmouseenter: create_set_cursor_handler(CursorIcon::NeResize),
			onmouseleave: reset_cursor_handler,
		}
		// Bottom-Left
		rect {
			position: "absolute", layer: "overlay",
			position_bottom: "0", position_left: "0",
			width: "{border_size}", height: "{border_size}",
			onmousedown: create_resize_handler(ResizeDirection::SouthWest),
			onmouseenter: create_set_cursor_handler(CursorIcon::SwResize),
			onmouseleave: reset_cursor_handler,
		}
		// Bottom-Right
		rect {
			position: "absolute", layer: "overlay",
			position_bottom: "0", position_right: "0",
			width: "{border_size}", height: "{border_size}",
			onmousedown: create_resize_handler(ResizeDirection::SouthEast),
			onmouseenter: create_set_cursor_handler(CursorIcon::SeResize),
			onmouseleave: reset_cursor_handler,
		}

		// Window edges
		// Top
		rect {
			position: "absolute", layer: "overlay",
			position_top: "0", position_left: "{border_size}",
			height: "{border_size}",
			width: "calc(100% - {2 * border_size}px)",
			onmousedown: create_resize_handler(ResizeDirection::North),
			onmouseenter: create_set_cursor_handler(CursorIcon::NResize),
			onmouseleave: reset_cursor_handler,
		}
		// Bottom
		rect {
			position: "absolute", layer: "overlay",
			position_bottom: "0", position_left: "{border_size}",
			height: "{border_size}",
			width: "calc(100% - {2 * border_size}px)",
			onmousedown: create_resize_handler(ResizeDirection::South),
			onmouseenter: create_set_cursor_handler(CursorIcon::SResize),
			onmouseleave: reset_cursor_handler,
		}
		// Left
		rect {
			position: "absolute", layer: "overlay",
			position_top: "{border_size}", position_left: "0",
			width: "{border_size}",
			height: "calc(100% - {2 * border_size}px)",
			onmousedown: create_resize_handler(ResizeDirection::West),
			onmouseenter: create_set_cursor_handler(CursorIcon::WResize),
			onmouseleave: reset_cursor_handler,
		}
		// Right
		rect {
			position: "absolute", layer: "overlay",
			position_top: "{border_size}", position_right: "0",
			width: "{border_size}",
			height: "calc(100% - {2 * border_size}px)",
			onmousedown: create_resize_handler(ResizeDirection::East),
			onmouseenter: create_set_cursor_handler(CursorIcon::EResize),
			onmouseleave: reset_cursor_handler,
		}
	}
}
