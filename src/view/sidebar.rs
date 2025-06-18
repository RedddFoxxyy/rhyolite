use crate::{THEME_STORE, app_state::ui_stores::SHOW_SETTINGS_DROPUP};
use freya::prelude::*;

#[component]
pub fn side_bar() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	rsx!(rect {
		width: "60",
		height: "fill",
		background: "transparent",
		border: "0 2 0 0 inner { theme.surface0 }",
		side_bar_buttons{},
		if *SHOW_SETTINGS_DROPUP.read() {
			settings_drop_up {}
		}
	})
}

#[component]
fn side_bar_buttons() -> Element {
	rsx!(rect {
		direction: "vertical",
		width: "100%",
		height: "100%",
		main_align: "space-between",
		top_buttons { },
		bottom_buttons {  }
	})
}

#[component]
fn top_buttons() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	rsx!(rect {
		direction: "vertical",
		width: "100%",
		height: "auto",
		spacing: "2",
		margin: "6 0 0 0",

		// Command Pallete Toggle Button
		sidebar_reactive_button {
			on_click: move |_| {THEME_STORE.write().toggle_theme_test()},
			svg {
				width: "100%",
				height: "100%",
				stroke: "{ theme.surface2 }",
				svg_content: include_str!("../static/svgs/command_palette.svg")
			}
		},

		// Recent Files Toggle Button
		sidebar_reactive_button {
			on_click: move |_| return,
			svg {
				width: "100%",
				height: "100%",
				stroke: "{ theme.surface2 }",
				svg_content: include_str!("../static/svgs/recent_files.svg")
			}
		}
	})
}

#[component]
fn bottom_buttons() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	rsx!(rect {
		direction: "vertical",
		width: "100%",
		height: "auto",
		spacing: "2",
		margin: "0 0 8 0",

		// Settings Toggle Button
		sidebar_reactive_button {
			on_click: move |_| {
				let current = *SHOW_SETTINGS_DROPUP.read();
				*SHOW_SETTINGS_DROPUP.write() = !current;
			},
			svg {
				width: "100%",
				height: "100%",
				stroke: "{ theme.surface2 }",
				svg_content: include_str!("../static/svgs/settings.svg")
			}
		},
	})
}

#[component]
fn settings_drop_up() -> Element {
	let theme = THEME_STORE().current_theme.colors;
	let themes_list = &THEME_STORE().store;
	rsx!(
		rect {
			position: "global",
			width: "220",
			height: "320",
			position_bottom: "10",
			position_left: "65",
			padding: "8 10",
			background: "{theme.base}",
			corner_radius: "20",
			ScrollView {
				width: "fill",
				direction: "vertical",
				spacing: "10",
				for key in themes_list.keys().cloned() {
					drop_up_item {
						label: key,
					}
				}
			}
		}
	)
}

#[component]
fn drop_up_item(label: String) -> Element {
	let theme = THEME_STORE().current_theme.colors;
	let mut hovered = use_signal(|| false);
	let background = if *hovered.read() {
		theme.surface2.to_string()
	} else {
		"transparent".to_string()
	};
	let label_clone = label.clone();

	rsx!(
		rect {
			width: "fill",
			height: "auto",
			background: "{background}",
			corner_radius: "10",
			padding: "2 5",
			key: "{label}",
			onclick: move |_| THEME_STORE.write().change_current_theme(&label_clone),
			onmouseenter: move |_| hovered.set(true),
			onmouseleave: move |_| hovered.set(false),
			label {
				color:"{ theme.text }",
				font_size: "18",
				"{label}"
			}
		}
	)
}

#[component]
fn sidebar_reactive_button(on_click: EventHandler<()>, children: Element) -> Element {
	let theme = THEME_STORE().current_theme.colors;

	let mut hovered = use_signal(|| false);

	let background = if *hovered.read() {
		theme.surface2.to_string()
	} else {
		"transparent".to_string()
	};

	let background_opacity_hover = if *hovered.read() {
		"0.2".to_string()
	} else {
		"0.0".to_string()
	};

	rsx!(
		rect {
			width: "100%",
			height: "50",
			padding: "3.5 9",
			main_align: "center",
			cross_align: "center",
			rect {
				background: "{background}",
				width: "100%",
				height: "100%",
				padding: "1.2",
				main_align: "center",
				cross_align: "center",
				background_opacity:"{background_opacity_hover}",
				corner_radius: "10",
				onclick: move |_| on_click.call(()),
				onmouseenter: move |_| hovered.set(true),
				onmouseleave: move |_| hovered.set(false),
				{children}
			}
		}
	)
}
