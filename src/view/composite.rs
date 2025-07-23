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

	// NOTE: Do not run this here, I am still figuring out how to deinitialise the app correctly,
	// for not I am running this funciton in docview/document_editor.
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

	// --- Resize Handlers ---

	let resize_right = move |_| {
		platform.with_window(|window| {
			let _ = window.drag_resize_window(ResizeDirection::East);
		});
	};
	let resize_left = move |_| {
		platform.with_window(|window| {
			let _ = window.drag_resize_window(ResizeDirection::West);
		});
	};
	let resize_top = move |_| {
		platform.with_window(|window| {
			let _ = window.drag_resize_window(ResizeDirection::North);
		});
	};
	let resize_bottom = move |_| {
		platform.with_window(|window| {
			let _ = window.drag_resize_window(ResizeDirection::South);
		});
	};
	let resize_top_left = move |_| {
		platform.with_window(|window| {
			let _ = window.drag_resize_window(ResizeDirection::NorthWest);
		});
	};
	let resize_top_right = move |_| {
		platform.with_window(|window| {
			let _ = window.drag_resize_window(ResizeDirection::NorthEast);
		});
	};
	let resize_bottom_left = move |_| {
		platform.with_window(|window| {
			let _ = window.drag_resize_window(ResizeDirection::SouthWest);
		});
	};
	let resize_bottom_right = move |_| {
		platform.with_window(|window| {
			let _ = window.drag_resize_window(ResizeDirection::SouthEast);
		});
	};

	rsx! {


		// Top-Left Corner
		CursorArea {
			icon: CursorIcon::NwResize,
			rect {
				position: "global",
				position_left: "0",
				position_top: "0",
				height: "12",
				width: "12",
				layer: "overlay",
				onmousedown: resize_top_left,
				onmouseenter: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::NwResize);
					})
				},
				onmouseleave: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::Default);
					})
				},
			}
		},
		// Top-Right Corner
		CursorArea {
			icon: CursorIcon::NeResize,
			rect {
				position: "global",
				position_right: "0",
				position_top: "0",
				height: "8",
				width: "8",
				layer: "overlay",
				// background: "black", //for debugging
				onmousedown: resize_top_right,
				onmouseenter: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::NeResize);
					})
				},
				onmouseleave: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::Default);
					})
				},
			}
		},
		// Bottom-Left Corner
		CursorArea {
			icon: CursorIcon::SwResize,
			rect {
				position: "global",
				position_left: "0",
				position_bottom: "0",
				height: "12",
				width: "12",
				layer: "overlay",
				onmousedown: resize_bottom_left,
				onmouseenter: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::SwResize);
					})
				},
				onmouseleave: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::Default);
					})
				},
			}
		},
		// Bottom-Right Corner
		CursorArea {
			icon: CursorIcon::SeResize,
			rect {
				position: "global",
				position_right: "0",
				position_bottom: "0",
				height: "12",
				width: "12",
				layer: "overlay",
				onmousedown: resize_bottom_right,
				onmouseenter: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::SeResize);
					})
				},
				onmouseleave: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::Default);
					})
				},
			}
		},
		// Right Edge
		CursorArea {
			icon: CursorIcon::EResize,
			rect {
				position: "global",
				position_right: "0",
				position_top: "0",
				height: "fill",
				width: "3",
				layer: "overlay",
				onmousedown: resize_right,
				onmouseenter: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::EResize);
					})
				},
				onmouseleave: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::Default);
					})
				},
			}
		},
		// Left Edge
		CursorArea {
			icon: CursorIcon::WResize,
			rect {
				position: "global",
				position_left: "0",
				position_top: "0",
				height: "fill",
				width: "3",
				layer: "overlay",
				onmousedown: resize_left,
				onmouseenter: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::WResize);
					})
				},
				onmouseleave: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::Default);
					})
				},
			}
		},
		// Top Edge
		CursorArea {
			icon: CursorIcon::NResize,
			rect {
				position: "global",
				position_right: "0",
				position_top: "0",
				height: "4",
				width: "fill",
				layer: "overlay",
				onmousedown: resize_top,
				onmouseenter: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::NResize);
					})
				},
				onmouseleave: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::Default);
					})
				},
			}
		},
		// Bottom Edge
		CursorArea {
			icon: CursorIcon::SResize,
			rect {
				position: "global",
				position_right: "0",
				position_bottom: "0",
				height: "5",
				width: "fill",
				layer: "overlay",
				onmousedown: resize_bottom,
				onmouseenter: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::SResize);
					})
				},
				onmouseleave: move |_| {
					platform.with_window(move |window| {
							window.set_cursor(CursorIcon::Default);
					})
				},
			}
		},
	}
}
