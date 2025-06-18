use crate::{APP_ICON, THEME_STORE};
use freya::prelude::*;

#[component]
pub fn top_nav_bar() -> Element {
	// let background_color = &app_state_hook.workspace.current_theme.colors.base;
	let theme = THEME_STORE().current_theme.colors;

	rsx!(
		// Will be used when this is fixed!
		// WindowDragArea {}
		rect {
			width: "100%",
			height: "50",
			direction: "horizontal",
			main_align: "space-between",
			cross_align: "center",
			background: "{ theme.base }",

			// 1. First Child: Aligned to the LEFT
			ActiveTabs {},

			// 2. Second Child: Aligned to the RIGHT
			// NavigationButtons {}
		}
	)
}

#[component]
fn ActiveTabs() -> Element {
	// let text_color = use_memo(move || APP_THEME.read().colors.text.clone());

	// let image_data = static_bytes(APP_ICON);
	rsx!(
		rect {
			direction: "horizontal",
			cross_align: "center",
			padding: "0 0 0 65",
			// spacing: "5",

			// App Icon Section
			// rect {
			// 	height: "80%",
			// 	width: "60",
			// 	cross_align: "center",
			// 	image {
			// 		width: "100%",
			// 		height: "100%",
			// 		image_data
			// 	}
			// },

			// Tabs Section
			rect {
				height: "fill",
				width: "auto",
				main_align: "center",
				tab_button {
					on_click: move |_| return,
				}
			}
		}
	)
}

#[component]
fn tab_button(on_click: EventHandler<()>, children: Element) -> Element {
	let theme = THEME_STORE().current_theme.colors;

	let mut hovered = use_signal(|| false);

	// let background = if *hovered.read() {
	// 	hover_color.to_string()
	// } else {
	// 	"transparent".to_string()
	// };

	let background_opacity_hover = if *hovered.read() {
		"1.0".to_string()
	} else {
		"0.6".to_string()
	};

	rsx!(
		rect {
			width: "160",
			height: "75%",
			padding: "2 18 2 20",
			direction: "horizontal",
			main_align: "space-between",
			cross_align: "center",
			background: "{ theme.surface1 }",
			background_opacity:"{background_opacity_hover}",
			corner_radius: "50",
			onclick: move |_| on_click.call(()),
			onmouseenter: move |_| hovered.set(true),
			onmouseleave: move |_| hovered.set(false),
			label {
				color: "{ theme.text }",
				font_size: "22",
				"Untitled"
			},
			label {
				color: "{ theme.text }",
				font_size: "20",
				"×"
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

	let theme = THEME_STORE().current_theme.colors;

	rsx!(rect {
		direction: "horizontal",
		width: "auto",
		height: "100%",
		NavButton {
			on_click: move |_| platform.toggle_minimize_window(),
			hover_color: "{ theme.surface2 }",
			svg {
				width: "100%",
				height: "60%",
				stroke: "{ theme.text }",
				fill: "{ theme.text }",
				svg_content: include_str!("../static/svgs/minimise.svg")
			}
		},
		NavButton {
			on_click: move |_| platform.toggle_maximize_window(),
			hover_color: theme.surface2,
			svg {
				width: "90%",
				height: "60%",
				stroke: "{ theme.text }",
				fill: "{ theme.text }",
				svg_content: include_str!("../static/svgs/maximise.svg")
			}
		},
		NavButton {
			on_click: move |_| platform.exit(),
			hover_color: "red",
			svg {
				width: "90%",
				height: "60%",
				stroke: "{ theme.text }",
				fill: "{ theme.text }",
				svg_content: include_str!("../static/svgs/close.svg")
			}
		}
	})
}
