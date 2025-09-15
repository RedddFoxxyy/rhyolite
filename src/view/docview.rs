use std::hash::{DefaultHasher, Hash, Hasher};

use crate::{
	data::{
		fn_utils::handle_editor_key_input,
		io_utils::{deinitialise_app, update_document_title},
		stores::{ACTIVE_DOCUMENT_TITLE, CURRENT_EDITOR_BUFFER, THEME_STORE},
	},
	view::bottom_bar::bottom_floating_bar,
};
use freya::prelude::*;
use tokio::time::Duration;
use tokio::time::sleep;

#[component]
pub fn work_space() -> Element {
	rsx!(rect {
		width: "fill",
		height: "fill",
		editor_area{}
		bottom_floating_bar {}
	})
}

#[component]
fn editor_area() -> Element {
	rsx!(rect {
		width: "fill",
		height: "fill",
		direction: "vertical",
		title_box{}
		editor_box_dynamic{}
	})
}

#[component]
fn title_box() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	let mut focus = use_focus();

	let mut editable = use_editable(
		|| EditableConfig::new("Untitled".trim().to_string()).with_allow_tabs(true),
		EditableMode::MultipleLinesSingleEditor,
	);

	use_effect(move || {
		editable.editor_mut().write().set(ACTIVE_DOCUMENT_TITLE().as_str());
		log::debug!("Document title updated");
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
		if e.data.key == Key::Enter {
			let new_title = editable.editor().read().to_string();
			spawn(async move {
				update_document_title(new_title).await;
			});
			focus.request_unfocus();
		}
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
			sleep(Duration::from_millis(550)).await;
			if focus.is_focused() {
				is_cursor_blinking.toggle();
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
		max_height: "100",
		main_align: "center",
		cross_align: "center",
		padding: "7",
		margin: "16 0 0 0",
		rect {
			width: "40%",
			min_width: "280",
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
					font_size: "40",
					font_family: "JetBrains Mono",
					text_overflow: "ellipsis",
					max_lines: "1",
					text {
						"{editable.editor()}"
					}
				}
			}
		}
	})
}

#[allow(dead_code)]
#[component]
fn editor_box() -> Element {
	let mut focus = use_focus();
	let mut editable = CURRENT_EDITOR_BUFFER();
	let editor = editable.editor().read();
	let mut is_cursor_blinking = use_signal(|| false);
	let theme = THEME_STORE().current_theme.colors;

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

	let scrollbar_theme = theme_with!(ScrollBarTheme {
		background: cow_borrowed!("transparent"), //
		thumb_background: Cow::from(theme.surface0.clone()),
		hover_thumb_background: Cow::from(theme.surface1.clone()),
		active_thumb_background: Cow::from(theme.surface2.clone()),
	});

	use_future(move || async move {
		loop {
			sleep(Duration::from_millis(500)).await;

			if focus.is_focused() {
				is_cursor_blinking.toggle();
			}
		}
	});

	// NOTE: This probably is not the correct place to run this function, however it works
	// correctly here, so for now the deinitialisation of the app run here.
	use_drop(move || {
		deinitialise_app();
	});

	rsx!(rect{
		width: "fill",
		height: "fill",
		cross_align: "center",
		padding: "7",
		margin: "16 0 0 0",
		CursorArea {
			icon: CursorIcon::Text,
			rect {
				width: "80%",
				height: "fill",
				background: "transparent",
				padding: "4 0",
				onkeydown,
				onkeyup,
				a11y_id: focus.attribute(),
				onclick,
				VirtualScrollView {
					width: "100%",
					height: "100%",
					length: editor.len_lines(),
					item_size: 25.0,
					show_scrollbar: true,
					scroll_with_arrows: true,
					scrollbar_theme,
					cache_elements: true,
					builder: move |line_index, _: &Option<()>| {
						let theme = THEME_STORE().current_theme.colors;
						let editor = editable.editor().read();
						let line = editor.line(line_index).unwrap();
						let is_line_selected = editor.cursor_row() == line_index;
						// Only show the cursor in the active line
						let character_index = if is_line_selected {
							editor.cursor_col().to_string()
						} else {
							"none".to_string()
						};
						let cursor_color = if focus.is_focused() && *is_cursor_blinking.read() {
							theme.text.as_str()
						} else {
							"transparent"
						};

						// Only highlight the active line
						// let line_background = if is_line_selected {
						// 	theme.subtext1.as_str()
						// } else {
						// 	"none"
						// };

						let line_background = "none";

						let onmousedown = move |e: MouseEvent| {
							focus.request_focus();
							editable.process_event(&EditableEvent::MouseDown(e.data, line_index));
						};

						let onmousemove = move |e: MouseEvent| {
							editable.process_event(&EditableEvent::MouseMove(e.data, line_index));
						};

						let highlights = editable.highlights_attr(line_index);

						rsx! {
							rect {
								key: "{line_index}",
								width: "100%",
								height: "23",
								// content: "fit",
								direction: "horizontal",
								background: "{line_background}",
								// Uncomment this for line numbers like that in codemirror.
								// label {
								// 	main_align: "center",
								// 	width: "30",
								// 	height: "100%",
								// 	text_align: "center",
								// 	font_size: "15",
								// 	color: "rgb(200, 200, 200)",
								// 	"{line_index + 1} "
								// }
								paragraph {
									cursor_reference: editable.cursor_attr(),
									main_align: "center",
									height: "100%",
									width: "98.5%",
									cursor_index: "{character_index}",
									cursor_color: "{cursor_color}",
									highlight_color: "{theme.subtext1}",
									max_lines: "1",
									cursor_mode: "editable",
									cursor_id: "{line_index}",
									onmousedown,
									onmousemove,
									highlights,
									text {
										color: "{theme.text}",
										font_size: "16",
										font_family: "JetBrains Mono",
										"{line}"
									}
								}
							}
						}
					}
				}
			}
		}
	})
}

#[allow(dead_code)]
#[component]
fn editor_box_dynamic() -> Element {
	let mut focus = use_focus();
	let mut editable = CURRENT_EDITOR_BUFFER();
	let mut is_cursor_blinking = use_signal(|| false);
	let theme = THEME_STORE().current_theme.colors;

	let onclick = move |_: MouseEvent| {
		focus.request_focus();
		*is_cursor_blinking.write() = true;
		editable.process_event(&EditableEvent::Click);
	};

	let onkeydown = {
		move |e: KeyboardEvent| {
			if handle_editor_key_input(&e) {
				editable.process_event(&EditableEvent::KeyDown(e.data));
			}
		}
	};

	let onkeyup = {
		move |e: KeyboardEvent| {
			if handle_editor_key_input(&e) {
				editable.process_event(&EditableEvent::KeyUp(e.data));
			}
		}
	};

	let scrollbar_theme = theme_with!(ScrollBarTheme {
		background: cow_borrowed!("transparent"), //
		thumb_background: Cow::from(theme.surface0.clone()),
		hover_thumb_background: Cow::from(theme.surface1.clone()),
		active_thumb_background: Cow::from(theme.surface2.clone()),
	});

	use_effect(move || {
		if focus.is_focused() {
			*is_cursor_blinking.write() = true;
			spawn(async move {
				while focus.is_focused() {
					sleep(Duration::from_millis(500)).await;
					if focus.is_focused() {
						is_cursor_blinking.toggle();
					}
				}
				*is_cursor_blinking.write() = false;
			});
		} else {
			*is_cursor_blinking.write() = false;
		}
	});

	// NOTE: This probably is not the correct place to run this function, however it works
	// correctly here, so for now the deinitialise function run here.
	// TODO: Run this use_drop only if window decorations are enabled.
	// use_drop(move || {
	// 	deinitialise_app();
	// });

	// Generate a unique and stable key for each line by hashing its content.
	// Required by dynamic scroll view.
	let item_keys = use_memo(use_reactive(&editable, move |editable| {
		let editor = editable.editor().read();
		(0..editor.len_lines())
			.map(|i| {
				let mut hasher = DefaultHasher::new();
				if let Some(line) = editor.line(i) {
					line.text.hash(&mut hasher);
				}
				i.hash(&mut hasher);
				hasher.finish()
			})
			.collect::<Vec<u64>>()
	}));

	rsx!(rect{
		width: "fill",
		height: "fill",
		cross_align: "center",
		padding: "7",
		margin: "16 0 40 0",
		CursorArea {
			icon: CursorIcon::Text,
			rect {
				width: "80%",
				height: "fill",
				background: "transparent",
				padding: "4 0",
				onkeydown,
				onkeyup,
				a11y_id: focus.attribute(),
				onclick,
				DynamicVirtualScrollView {
					width: "100%",
					height: "100%",
					padding: "0",
					default_item_height: 5.0,
					overscan: 3,
					scroll_with_arrows: true,
					scroll_beyond_last_item: 10,
					min_scrollthumb_height: Some(25.0),
					item_keys: item_keys(),
					scrollbar_theme,
					builder: move |line_index| {
						let theme = THEME_STORE().current_theme.colors;
						let editor = editable.editor().read();

						let line = match editor.line(line_index) {
							Some(line) => line,
							None => return rsx! { rect {} }
						};

						let is_line_selected = editor.cursor_row() == line_index;
						let character_index = if is_line_selected {
							editor.cursor_col().to_string()
						} else {
							"none".to_string()
						};
						let cursor_color = if focus.is_focused() && *is_cursor_blinking.read() {
							theme.text.as_str()
						} else {
							"transparent"
						};
						let line_background = "none";

						let onmousedown = move |e: MouseEvent| {
							editable.process_event(&EditableEvent::MouseDown(e.data, line_index));
						};

						let onmousemove = move |e: MouseEvent| {
							editable.process_event(&EditableEvent::MouseMove(e.data, line_index));
						};

						let highlights = editable.highlights_attr(line_index);

						rsx! {
							rect {
								width: "100%",
								height: "auto",
								content: "fit",
								direction: "horizontal",
								background: "{line_background}",
								paragraph {
									cursor_reference: editable.cursor_attr(),
									main_align: "center",
									height: "auto",
									width: "98.5%",
									cursor_index: "{character_index}",
									cursor_color: "{cursor_color}",
									highlight_color: "{theme.subtext1}",
									cursor_mode: "editable",
									cursor_id: "{line_index}",
									onmousedown,
									onmousemove,
									highlights,
									text {
										color: "{theme.text}",
										font_size: "16",
										font_family: "JetBrains Mono",
										"{line}"
									}
								}
							}
						}
					}
				}
			}
		}
	})
}
