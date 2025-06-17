use crate::{APP_THEME, GLOBAL_APP_STATE};
use freya::prelude::*;

#[component]
pub fn side_bar() -> Element {
	let border_color = use_memo(move || APP_THEME.read().colors.surface0.clone());

	rsx!(rect {
		width: "60",
		height: "100%",
		background: "transparent",
		border: "0 2 0 0 inner {border_color}",
		side_bar_buttons{}
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
	let text_color = use_memo(move || APP_THEME.read().colors.surface2.clone());

	rsx!(rect {
		direction: "vertical",
		width: "100%",
		height: "auto",
		spacing: "2",
		margin: "6 0 0 0",

		// Command Pallete Toggle Button
		reactive_button {
			on_click: move |_| return,
			svg {
				width: "100%",
				height: "100%",
				stroke: "{text_color}",
				svg_content: include_str!("../static/svgs/command_palette.svg")
			}
		},

		// Recent Files Toggle Button
		reactive_button {
			on_click: move |_| return,
			svg {
				width: "100%",
				height: "100%",
				stroke: "{text_color}",
				svg_content: include_str!("../static/svgs/recent_files.svg")
			}
		}
	})
}

#[component]
fn bottom_buttons() -> Element {
	let text_color = use_memo(move || APP_THEME.read().colors.surface2.clone());

	rsx!(rect {
		direction: "vertical",
		width: "100%",
		height: "auto",
		spacing: "2",
		margin: "0 0 58 0",

		// Settings Toggle Button
		reactive_button {
			on_click: move |_| return,
			svg {
				width: "100%",
				height: "100%",
				stroke: "{text_color}",
				svg_content: include_str!("../static/svgs/settings.svg")
			}
		},
	})
}

#[component]
fn reactive_button(on_click: EventHandler<()>, children: Element) -> Element {
	let hover_color = use_memo(move || APP_THEME.read().colors.surface2.clone());

	let mut hovered = use_signal(|| false);

	let background = if *hovered.read() {
		hover_color.to_string()
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
