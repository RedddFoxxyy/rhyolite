use crate::{
	data::{
		fn_utils::handle_editor_key_input,
		io_utils::deinitialise_app,
		stores::{
			doc_store::{ACTIVE_DOCUMENT_TITLE, CURRENT_EDITOR_BUFFER},
			ui_store::THEME_STORE,
		},
	},
	view::bottom_bar::bottom_floating_bar,
};
use freya::prelude::*;
use tokio::time::Duration;
use tokio::time::sleep;

#[component]
pub fn work_space() -> Element {
	rsx!(rect {
		// Take the entire window width and height
		width: "fill",
		height: "fill",
		document_area{},
		bottom_floating_bar {  }
	})
}

fn document_area() -> Element {
	rsx!(rect {
		width: "fill",
		height: "fill",
		direction: "vertical",
		document_title_box{}
		document_editor{}
	})
}

fn document_title_box() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	let mut focus = use_focus();

	let mut editable = use_editable(
		|| EditableConfig::new("Untitled".trim().to_string()).with_allow_tabs(true),
		EditableMode::MultipleLinesSingleEditor,
	);

	use_effect(move || {
		editable
			.editor_mut()
			.write()
			.set(ACTIVE_DOCUMENT_TITLE().as_str())
	});

	let cursor_reference = editable.cursor_attr();
	let highlights = editable.highlights_attr(0);
	let editor = editable.editor().read();
	let cursor_char = editor.cursor_pos();
	let mut is_cursor_blinking = use_signal(|| false);

	let onmousedown = move |e: MouseEvent| {
		focus.request_focus();
		editable.process_event(&EditableEvent::MouseDown(e.data, 0));
	};

	let onmousemove = move |e: MouseEvent| {
		editable.process_event(&EditableEvent::MouseMove(e.data, 0));
	};

	let onclick = move |_: MouseEvent| {
		editable.process_event(&EditableEvent::Click);
	};

	let onkeydown = move |e: KeyboardEvent| {
		if handle_editor_key_input(&e) {
			editable.process_event(&EditableEvent::KeyDown(e.data));
		}
	};

	let onkeyup = move |e: KeyboardEvent| {
		if handle_editor_key_input(&e) {
			editable.process_event(&EditableEvent::KeyUp(e.data));
		}
	};

	// A future that runs a timer to toggle the blink signal
	use_future(move || async move {
		loop {
			sleep(Duration::from_millis(500)).await;

			if focus.is_focused() {
				is_cursor_blinking.toggle();
			} else {
				*is_cursor_blinking.write() = false
			}
		}
	});

	let cursor_color = if focus.is_focused() && *is_cursor_blinking.read() {
		theme.text.as_str()
	} else {
		"transparent"
	};

	rsx!(rect{
		width: "fill",
		height: "15%",
		min_height: "80",
		max_height: "120",
		main_align: "center",
		cross_align: "center",
		padding: "7",
		margin: "16 0 0 0",
		rect {
			width: "40%",
			min_width: "270",
			height: "fill",
			shadow: "5 8 8 2 rgb(0, 0, 0, 10)",
			background: "{theme.base}",
			corner_radius: "12",
			main_align: "center",
			padding: "4 12",

			CursorArea {
				icon: CursorIcon::Text,
				paragraph {
					width: "fill",
					cursor_id: "0",
					cursor_index: "{cursor_char}",
					cursor_mode: "editable",
					cursor_color: "{cursor_color}",
					highlights,
					highlight_color: "{theme.subtext1}",
					a11y_id: focus.attribute(),
					cursor_reference,
					onclick,
					onmousemove,
					onmousedown,
					onkeydown,
					onkeyup,
					color: "{theme.text}",
					font_size: "42",
					font_family: "JetBrains Mono",
					text {
						"{editable.editor()}"
					}

				}
			}
		}
	})
}

// TODO: Handle multi line highlights by iterating through each editor and handling the highlight attribute
fn document_editor() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	let mut focus = use_focus();

	let mut editable = CURRENT_EDITOR_BUFFER();

	let cursor_reference = editable.cursor_attr();
	let highlights = editable.highlights_attr(0);
	let editor = editable.editor().read();
	let cursor_char = editor.cursor_pos();
	let mut is_cursor_blinking = use_signal(|| false);

	let onmousedown = move |e: MouseEvent| {
		focus.request_focus();
		editable.process_event(&EditableEvent::MouseDown(e.data, 0));
	};

	let onmousemove = move |e: MouseEvent| {
		editable.process_event(&EditableEvent::MouseMove(e.data, 0));
	};

	let onclick = move |_: MouseEvent| {
		*is_cursor_blinking.write() = true;
		editable.process_event(&EditableEvent::Click);
	};

	let onkeydown = move |e: KeyboardEvent| {
		if handle_editor_key_input(&e) {
			editable.process_event(&EditableEvent::KeyDown(e.data));
		}
	};

	let onkeyup = move |e: KeyboardEvent| {
		if handle_editor_key_input(&e) {
			editable.process_event(&EditableEvent::KeyUp(e.data));
		}
	};

	use_future(move || async move {
		loop {
			sleep(Duration::from_millis(500)).await;

			if focus.is_focused() {
				is_cursor_blinking.toggle();
			}
		}
	});

	// NOTE: This probably is not the correct place to run this function, however it works
	// correctly here, so for now the deinitialise function run here.
	use_drop(move || {
		deinitialise_app();
	});

	let cursor_color = if focus.is_focused() && *is_cursor_blinking.read() {
		theme.text.as_str()
	} else {
		"transparent"
	};

	rsx!(rect{
		width: "fill",
		height: "fill",
		cross_align: "center",
		padding: "7",
		margin: "16 0 0 0",
		rect {
			width: "80%",
			height: "fill",
			background: "transparent",
			corner_radius: "12",
			padding: "4 12",

			CursorArea {
				icon: CursorIcon::Text,
				paragraph {
					width: "fill",
					height: "fill",
					cursor_id: "0",
					cursor_index: "{cursor_char}",
					cursor_mode: "editable",
					cursor_color: "{cursor_color}",
					highlights,
					highlight_color: "{theme.subtext1}",
					a11y_id: focus.attribute(),
					cursor_reference,
					onclick,
					onmousemove,
					onmousedown,
					onkeydown,
					onkeyup,
					color: "{theme.text}",
					font_size: "16",
					font_family: "JetBrains Mono",
					text {
						"{editable.editor()}"
					},
				}
			}
		}
	})
}
