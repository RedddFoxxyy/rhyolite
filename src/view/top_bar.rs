#![allow(unused_imports)]
use crate::{
	APP_ICON,
	data::stores::{
		tabs_store::{CURRENT_TAB, TABS, close_tab, new_tab, switch_tab},
		ui_store::THEME_STORE,
	},
};
use freya::hooks::Window;
use freya::prelude::*;

#[component]
pub fn top_nav_bar() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	rsx!(
		rect {
			width: "100%",
			height: "40",
			direction: "horizontal",
			main_align: "space-between",
			cross_align: "center",
			background: "{ theme.base }",
			WindowDragArea {
				rect {
					width: "120",
					height: "100%"
				}
			}
			active_tabs {},
			// NOTE: Will be used when this is fixed!
			WindowDragArea {
				rect {
					width: "100%",
					height: "100%"
				}
			}
			NavigationButtons {}
		}
	)
}

#[component]
fn active_tabs() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	let mut is_hovered = use_signal(|| false);

	let hover_animation = use_animation(move |_conf| {
		// conf.auto_start(false);
		AnimNum::new(0.0, 0.99999).time(150)
	});

	let onmouseenter = move |_| {
		*is_hovered.write() = true;
		hover_animation.start();
	};

	let onmouseleave = move |_| {
		*is_hovered.write() = false;
		hover_animation.reverse();
	};

	let bg_hover_opacity = &*hover_animation.get().read_unchecked();

	// let image_data = static_bytes(APP_ICON);
	rsx!(
		rect {
			direction: "horizontal",
			cross_align: "center",
			// padding: "0 0 0 60",
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
			for (index, _tab) in TABS().iter().enumerate() {
				rect {
					height: "fill",
					width: "auto",
					main_align: "center",
					margin: "0 2",
					tab_button {
						index: index,
						on_click: move |_| switch_tab(index),
					}
				}
			}
			CursorArea {
				icon: CursorIcon::Pointer,
				rect {
					corner_radius: "100",
					padding: "4 10",
					margin: "1 2",
					background: "{ theme.surface1 }",
					background_opacity:"{ bg_hover_opacity.read() }",
					onmouseenter,
					onmouseleave,
					onclick: move |_| new_tab(),
					label {
						color: "{ theme.text }",
						font_size: "17",
						font_family: "JetBrains Mono",
						"+"
					}
				}
			}
		}
	)
}

#[component]
fn tab_button(index: usize, on_click: EventHandler<()>, children: Element) -> Element {
	let theme = THEME_STORE().current_theme.colors;
	// TODO: Handle Unwrap
	let title = TABS().get(index).unwrap().title.clone();

	let mut is_hovered = use_signal(|| false);

	let hover_animation = use_animation(move |_conf| {
		// conf.auto_start(false);
		if CURRENT_TAB() == Some(index) {
			AnimNum::new(0.5, 0.99999).time(150)
		} else {
			AnimNum::new(0.0, 0.99999).time(150)
		}
	});

	let onmouseenter = move |_| {
		*is_hovered.write() = true;
		hover_animation.start();
	};

	let onmouseleave = move |_| {
		*is_hovered.write() = false;
		hover_animation.reverse();
	};

	let bg_hover_opacity = &*hover_animation.get().read_unchecked();

	rsx!(
		CursorArea {
			icon: CursorIcon::Pointer,
			rect {
				width: "160",
				height: "75%",
				padding: "2 12 2 15",
				direction: "horizontal",
				main_align: "space-between",
				cross_align: "center",
				background: "{ theme.surface1 }",
				background_opacity:"{ bg_hover_opacity.read() }",
				corner_radius: "50",
				onclick: move |_| on_click.call(()),
				onmouseenter,
				onmouseleave,
				label {
					color: "{ theme.text }",
					font_size: "15",
					font_family: "JetBrains Mono",
					"{title}"
				},
				if CURRENT_TAB() == Some(index) || is_hovered() {
					label {
						color: "{ theme.text }",
						font_size: "17",
						font_family: "JetBrains Mono",
						onclick: move |_| close_tab(index),
						"×"
					}
				}
			}
		}
	)
}

#[component]
fn nav_button(on_click: EventHandler<()>, hover_color: String, children: Element) -> Element {
	let mut hovered = use_signal(|| false);

	let background = if *hovered.read() {
		hover_color
	} else {
		"transparent".to_string()
	};

	rsx!(
		rect {
			background: "{background}",
			width: "60",
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
	let mut is_maximised = use_signal_sync(|| false);

	// TODO: Trigger this whenever system resizes the window!
	// use_effect(move || {
	// 	platform.with_window(move |window| {
	// 		*is_maximised.write() = window.is_maximized();
	// 		println!("{}", is_maximised())
	// 	})
	// });

	let theme = THEME_STORE().current_theme.colors;

	rsx!(rect {
		direction: "horizontal",
		width: "auto",
		height: "100%",
		nav_button {
			on_click: move |_| platform.set_minimize_window(true),
			hover_color: "{ theme.surface2 }",
			svg {
				width: "100%",
				height: "60%",
				stroke: "{ theme.text }",
				fill: "{ theme.text }",
				svg_content: include_str!("../static/svgs/minimise.svg")
			}
		},
		nav_button {
			on_click: move |_| {
				platform.toggle_maximize_window();
				is_maximised.toggle();
			},
			hover_color: theme.surface2,
			if is_maximised() {
				svg {
					width: "90%",
					height: "60%",
					stroke: "{ theme.text }",
					fill: "{ theme.text }",
					svg_content: include_str!("../static/svgs/restore.svg")
				}
			} else {
				svg {
					width: "90%",
					height: "60%",
					stroke: "{ theme.text }",
					fill: "{ theme.text }",
					svg_content: include_str!("../static/svgs/maximise.svg")
				}
			}
		},
		nav_button {
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
