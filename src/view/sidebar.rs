use crate::data::stores::ui_store::{
	SHOW_SETTINGS_DROPUP, SHOW_THEMES_DROPUP, THEME_STORE, toggle_command_palette,
	toggle_recent_files, toggle_settings_dropup, toggle_themes_dropup,
};
use crate::view::dropdown;
use freya::prelude::*;

#[component]
pub fn side_bar() -> Element {
	let theme = THEME_STORE().current_theme.colors;
	let themes_list = THEME_STORE().store;

	let settings_list: [dropdown::ButtonProps; 4] = [
		dropdown::ButtonProps {
			label: "General Settings".to_string(),
			on_click: |_| return,
			icon: Some(include_str!("../static/svgs/sliders-horizontal.svg")),
		},
		dropdown::ButtonProps {
			label: "Theme".to_string(),
			on_click: |_| toggle_themes_dropup(),
			icon: Some(include_str!("../static/svgs/palette.svg")),
		},
		dropdown::ButtonProps {
			label: "Keyboard Shortcuts".to_string(),
			on_click: |_| return,
			icon: Some(include_str!("../static/svgs/keyboard.svg")),
		},
		dropdown::ButtonProps {
			label: "About".to_string(),
			on_click: |_| return,
			icon: Some(include_str!("../static/svgs/info.svg")),
		},
	];

	rsx!(rect {
		width: "50",
		height: "fill",
		background: "transparent",
		border: "0 2 0 0 outer { theme.surface0 }",
		side_bar_buttons{},
		if *SHOW_SETTINGS_DROPUP.read() {
			dropdown::menu {
				rect {
					for setting in settings_list {
						dropdown::button{..setting}
					}
				}
			}

			if *SHOW_THEMES_DROPUP.read() {
				dropdown::submenu {
					for theme in themes_list.keys().cloned() {
						dropdown::button {
							label: theme,
							on_click: |theme| THEME_STORE.write().change_current_theme(theme),
						}
					}
				}
			}
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
		spacing: "1",
		margin: "5 0 0 0",

		// Command Pallete Toggle Button
		sidebar_button {
			on_click: move |_| toggle_command_palette(),
			svg {
				width: "100%",
				height: "100%",
				stroke: "{ theme.surface2 }",
				svg_content: include_str!("../static/svgs/command_palette.svg")
			}
		},

		// Recent Files Toggle Button
		sidebar_button {
			on_click: move |_| toggle_recent_files(),
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
		spacing: "1",
		margin: "0 0 6 0",

		// Settings Toggle Button
		sidebar_button {
			on_click: toggle_settings_dropup,
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
fn sidebar_button(on_click: EventHandler<()>, children: Element) -> Element {
	// Transition state with duration of 150ms
	//
	// NOTE: Instead of using a hovered signal, we might hard code it
	// into the on_mouse_enter and on_mouse_leave closures.
	let animation = use_animation(move |_conf| {
		// conf.auto_start(false);
		(
			AnimColor::new("transparent", &THEME_STORE().current_theme.colors.surface2).time(150),
			AnimNum::new(0.0, 0.2).time(150),
		)
	});

	let (bg_color, hover_opacity) = &*animation.get().read_unchecked();

	rsx!(
		CursorArea {
			icon: CursorIcon::Pointer,
			rect {
				width: "100%",
				height: "40",
				padding: "2 6",
				main_align: "center",
				cross_align: "center",
				rect {
					background: "{bg_color.read()}",
					width: "100%",
					height: "100%",
					padding: "1.3",
					main_align: "center",
					cross_align: "center",
					background_opacity:"{hover_opacity.read()}",
					corner_radius: "8",
					onclick: move |_| on_click.call(()),
					onmouseenter: move |_| animation.start(),
					onmouseleave: move |_| animation.reverse(),
					{children}
				}
			}
		}

	)
}
