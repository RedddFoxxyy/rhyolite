use crate::data::stores::ui_store::{
	SHOW_SETTINGS_DROPUP, SHOW_THEMES_DROPUP, THEME_STORE, toggle_command_palette, toggle_recent_files, toggle_settings_dropup,
	toggle_themes_dropup,
};
use crate::view::components::buttons;
use crate::view::dropdown;
use freya::prelude::*;

#[component]
pub fn side_bar() -> Element {
	let theme = THEME_STORE().current_theme.colors;
	let themes_list = THEME_STORE().themes_list;
	let platform = use_platform();

	let settings_list: [buttons::DropDownButtonProps; 4] = [
		buttons::DropDownButtonProps {
			label: "General Settings".to_string(),
			onclick: EventHandler::new(move |_| {
				platform.new_window(
					WindowConfig::new_with_props(settings_window, settings_windowProps { value: 404 }).with_title("Rhyolite Settings"),
				)
			}),
			icon: Some(include_str!("../static/svgs/sliders-horizontal.svg")),
			..Default::default()
		},
		buttons::DropDownButtonProps {
			label: "Theme".to_string(),
			onclick: EventHandler::new(|_| {
				toggle_themes_dropup();
			}),
			icon: Some(include_str!("../static/svgs/palette.svg")),
			..Default::default()
		},
		buttons::DropDownButtonProps {
			label: "Keyboard Shortcuts".to_string(),
			onclick: EventHandler::new(|_| {}),
			icon: Some(include_str!("../static/svgs/keyboard.svg")),
			..Default::default()
		},
		buttons::DropDownButtonProps {
			label: "About".to_string(),
			onclick: EventHandler::new(|_| {}),
			icon: Some(include_str!("../static/svgs/info.svg")),
			..Default::default()
		},
	];

	rsx!(rect {
		width: "50",
		height: "fill",
		background: "transparent",
		border: "0 2 0 0 outer { theme.surface0 }",
		side_bar_buttons{},
		if SHOW_SETTINGS_DROPUP() {
			dropdown::menu {
				rect {
					for setting in settings_list {
						buttons::DropDownButton{..setting}
					}
				}
			}
			if SHOW_THEMES_DROPUP() {
				dropdown::submenu {
					for theme in themes_list {
						buttons::DropDownButton {
							label: &theme,
							onclick: EventHandler::new({
								let theme = theme.clone();
								move |_| {
									let theme_clone = theme.clone();
									spawn(async move {
										THEME_STORE.write().change_current_theme(theme_clone).await;
									});
								}
							}),
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

		// Command Palette Toggle Button
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

// TODO: Move this into its own component.
#[component]
fn settings_window(value: i32) -> Element {
	let platform = use_platform();
	let theme = THEME_STORE().current_theme.colors;

	let onpress = move |_| platform.close_window();

	rsx!(
		rect {
			height: "100%",
			width: "100%",
			main_align: "center",
			cross_align: "center",
			background: theme.crust,
			font_size: "30",
			label {
				color: theme.text,
				"{value}: Yet to be Implemented."
			}
			Button {
				onpress,
				label { "Close" }
			}
		}
	)
}
