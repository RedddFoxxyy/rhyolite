use std::{default, os::unix::process::parent_id};

use crate::{data::ui::THEME_STORE, view::bottom_bar::bottom_floating_bar};
use freya::prelude::{dioxus_elements::attributes::padding, *};

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
	})
}

fn document_title_box() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	let mut editable = use_editable(
		|| EditableConfig::new("Untitled".trim().to_string()).with_allow_tabs(true),
		EditableMode::MultipleLinesSingleEditor,
	);

	let cursor_reference = editable.cursor_attr();
	let highlights = editable.highlights_attr(0);
	let editor = editable.editor().read();
	let cursor_char = editor.cursor_pos();

	let onmousedown = move |e: MouseEvent| {
		editable.process_event(&EditableEvent::MouseDown(e.data, 0));
	};

	let onmousemove = move |e: MouseEvent| {
		editable.process_event(&EditableEvent::MouseMove(e.data, 0));
	};

	let onclick = move |_: MouseEvent| {
		editable.process_event(&EditableEvent::Click);
	};

	let onglobalkeydown = move |e: KeyboardEvent| {
		editable.process_event(&EditableEvent::KeyDown(e.data));
	};

	let onglobalkeyup = move |e: KeyboardEvent| {
		editable.process_event(&EditableEvent::KeyUp(e.data));
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
					cursor_color: "{theme.text}",
					highlights,
					highlight_color: "{theme.subtext1}",
					cursor_reference,
					onclick,
					onmousemove,
					onmousedown,
					onglobalkeydown,
					onglobalkeyup,
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
