use std::hash::{DefaultHasher, Hash, Hasher};

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
		width: "fill",
		height: "fill",
		document_area{}
		bottom_floating_bar {  }
	})
}

fn document_area() -> Element {
	rsx!(rect {
		width: "fill",
		height: "fill",
		direction: "vertical",
		document_title_box{}
		// document_editor{}
		// document_editor_virtualised{}
		document_editor_dynamic_virtualised{}
	})
}

fn document_title_box() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	let mut focus = use_focus();

	let mut editable = use_editable(
		|| EditableConfig::new("Untitled".trim().to_string()).with_allow_tabs(true),
		EditableMode::MultipleLinesSingleEditor,
	);

	use_effect(move || editable.editor_mut().write().set(ACTIVE_DOCUMENT_TITLE().as_str()));

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
fn document_editor() -> Element {
	let mut focus = use_focus();
	let mut editable = CURRENT_EDITOR_BUFFER();
	let editor = editable.editor().read();
	let mut is_cursor_blinking = use_signal(|| false);

	let onmousedown = move |_e: MouseEvent| {
		focus.request_focus();
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

	rsx!(rect{
		width: "fill",
		height: "fill",
		cross_align: "center",
		padding: "7",
		margin: "16 0 10 0",
		overflow: "none",
		CursorArea {
			icon: CursorIcon::Text,
		rect {
			width: "85%",
			height: "fill",
			background: "transparent",
			padding: "4 0",
			onmousedown,
			onkeydown,
			onkeyup,
			a11y_id: focus.attribute(),
			onclick,
			overflow: "scroll",
			// NOTE: I know VirtualScrollView is more efficient, but I need text wrapping.
			ScrollView {
				width: "100%",
				height: "100%",
				show_scrollbar: true,
				scrollbar_theme: theme_with!(
					ScrollBarTheme {
						background: cow_borrowed!("transparent")
					}
				),

				for line_index in 0..editor.len_lines() {
					{
						let theme = THEME_STORE().current_theme.colors;
						let editor = editable.editor().read();
						let line = editor.line(line_index).unwrap();
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
							// focus.request_focus();
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
									height: "auto",
									// min_height: "12",
									// margin: "0",
									overflow: "none",
									content: "fit",
									direction: "horizontal",
									background: "{line_background}",
									paragraph {
										cursor_reference: editable.cursor_attr(),
										main_align: "center",
										height: "auto",
										width: "100%",
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
											// line_height: "0.5",
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
		}
	})
}

#[allow(dead_code)]
fn document_editor_virtualised() -> Element {
	let mut focus = use_focus();
	let mut editable = CURRENT_EDITOR_BUFFER();
	let editor = editable.editor().read();
	let mut is_cursor_blinking = use_signal(|| false);

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
					scroll_with_arrows: false,
					cache_elements: false,
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
									width: "100%",
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
fn document_editor_dynamic_virtualised() -> Element {
	let mut focus = use_focus();
	let mut editable = CURRENT_EDITOR_BUFFER();
	let mut is_cursor_blinking = use_signal(|| false);

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

	// Generate a unique and stable key for each line by hashing its content.
	let editor = editable.editor().read();
	let item_keys: Vec<u64> = (0..editor.len_lines())
		.map(|i| {
			let mut hasher = DefaultHasher::new();
			if let Some(line) = editor.line(i) {
				line.text.hash(&mut hasher);
			}
			hasher.finish()
		})
		.collect();

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
					overscan: 3,
					scroll_with_arrows: true,
					item_keys: item_keys,
					scrollbar_theme: theme_with!(
						ScrollBarTheme {
							background: cow_borrowed!("transparent")
						}
					),
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
								key: "{line_index}",
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
