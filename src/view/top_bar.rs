use crate::{APP_ICON, APP_THEME};
use freya::prelude::*;

#[component]
pub fn top_nav_bar() -> Element {
	// let background_color = &app_state_hook.workspace.current_theme.colors.base;
	let background_color = use_memo(move || APP_THEME.read().colors.base.clone());

	rsx!(
		// Will be used when this is fixed!
		// WindowDragArea {}
		rect {
			width: "100%",
			height: "50",
			direction: "horizontal",
			main_align: "space-between",
			cross_align: "center",
			background: "{background_color}",

			// 1. First Child: Aligned to the LEFT
			ActiveTabs {},

			// 2. Second Child: Aligned to the RIGHT
			// NavigationButtons {}
		}
	)
}

#[component]
fn ActiveTabs() -> Element {
	let text_color = use_memo(move || APP_THEME.read().colors.text.clone());

	let image_data = static_bytes(APP_ICON);
	rsx!(
		rect {
			width: "60", // Shrink to fit the content
			padding: "5",
			direction: "horizontal",
			main_align: "center",
			cross_align: "center",
			rect {
				height: "100%",
				width: "35",
				image {
					width: "100%",
					height: "100%",
					image_data
				}
			}
		}
	)
}

#[component]
fn NavButton(on_click: EventHandler<()>, hover_color: String, children: Element) -> Element {
	let mut hovered = use_signal(|| false);

	let background = if *hovered.read() {
		hover_color
	} else {
		"transparent".to_string()
	};

	rsx!(
		rect {
			background: "{background}",
			width: "62",
			height: "100%",
			padding: "3.5",
			main_align: "center",
			cross_align: "center",
			onclick: move |_| on_click.call(()),
			onmouseenter: move |_| hovered.set(true),
			onmouseleave: move |_| hovered.set(false),
			{children}
		}
	)
}

#[component]
fn NavigationButtons() -> Element {
	let platform = use_platform();

	let text_color = use_memo(move || APP_THEME.read().colors.text.clone());

	let hover_color = use_memo(move || APP_THEME.read().colors.surface2.clone());

	rsx!(rect {
		direction: "horizontal",
		width: "auto",
		height: "100%",
		NavButton {
			on_click: move |_| platform.toggle_minimize_window(),
			hover_color: hover_color,
			svg {
				width: "100%",
				height: "60%",
				stroke: "{text_color}",
				fill: "{text_color}",
				svg_content: include_str!("../static/svgs/minimise.svg")
			}
		},
		NavButton {
			on_click: move |_| platform.toggle_maximize_window(),
			hover_color: hover_color,
			svg {
				width: "90%",
				height: "60%",
				stroke: "{text_color}",
				fill: "{text_color}",
				svg_content: include_str!("../static/svgs/maximise.svg")
			}
		},
		NavButton {
			on_click: move |_| platform.exit(),
			hover_color: "red",
			svg {
				width: "90%",
				height: "60%",
				stroke: "{text_color}",
				fill: "{text_color}",
				svg_content: include_str!("../static/svgs/close.svg")
			}
		}
	})
}
