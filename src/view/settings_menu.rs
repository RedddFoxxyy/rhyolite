use crate::data::ui::{
	SHOW_COMMAND_PALETTE, SHOW_RECENT_FILES, SHOW_SETTINGS_DROPUP, SHOW_THEMES_DROPUP, THEME_STORE,
	toggle_command_palette, toggle_recent_files, toggle_settings_dropup, toggle_themes_dropup,
};
use freya::prelude::*;

#[derive(PartialEq, Clone)]
struct SettingsDropUpItem {
	name: String,
	on_click: fn(),
}

#[component]
pub fn settings_drop_up() -> Element {
	let theme = THEME_STORE().current_theme.colors;
	let settings_dropup_list: [SettingsDropUpItem; 4] = [
		SettingsDropUpItem {
			name: "General Settings".to_string(),
			on_click: || return,
		},
		SettingsDropUpItem {
			name: "Theme".to_string(),
			on_click: || toggle_themes_dropup(),
		},
		SettingsDropUpItem {
			name: "Keyboard Shortcuts".to_string(),
			on_click: || return,
		},
		SettingsDropUpItem {
			name: "About".to_string(),
			on_click: || return,
		},
	];
	// height: "auto",
	// position_bottom: "150",
	rsx!(
		rect {
			position: "global",
			width: "220",
			height: "160",
			position_bottom: "10",
			position_left: "65",
			padding: "8 13 8 10",
			background: "{theme.base}",
			corner_radius: "12",
			rect {
				width: "fill",
				direction: "vertical",
				spacing: "6",
				for item in settings_dropup_list {
					settings_dropup_button {
						item,
					}
				}
			}
		},
		if *SHOW_THEMES_DROPUP.read() {
			themes_drop_up {  }
		}
	)
}

#[component]
fn themes_drop_up() -> Element {
	let theme = THEME_STORE().current_theme.colors;
	let themes_list = &THEME_STORE().store;
	rsx!(
		rect {
			position: "global",
			width: "220",
			height: "320",
			position_bottom: "10",
			position_left: "290",
			padding: "8 13 8 10",
			background: "{theme.base}",
			corner_radius: "12",
			ScrollView {
				width: "fill",
				direction: "vertical",
				spacing: "6",
				for key in themes_list.keys().cloned() {
					theme_dropup_button {
						label: key,
					}
				}
			}
		}
	)
}

#[component]
fn theme_dropup_button(label: String) -> Element {
	let theme = THEME_STORE().current_theme.colors;
	let mut hovered = use_signal(|| false); // required in future

	let animation = use_animation(move |conf| {
		conf.auto_start(false);

		AnimColor::new(
			&THEME_STORE().current_theme.colors.base,
			&THEME_STORE().current_theme.colors.surface1,
		)
		.time(150)
	});

	let bg_color = &*animation.get().read_unchecked();

	let label_clone = label.clone();

	rsx!(
		CursorArea {
			icon: CursorIcon::Pointer,
			rect {
				width: "fill",
				height: "auto",
				background: "{bg_color.read()}",
				corner_radius: "10",
				padding: "5 6",
				onclick: move |_| THEME_STORE.write().change_current_theme(&label_clone),
				onmouseenter: move |_| {
					hovered.set(true);
					animation.start();
				},
				onmouseleave: move |_| {
					hovered.set(false);
					animation.reverse();
				},
				label {
					color:"{ theme.text }",
					font_size: "16",
					font_family: "JetBrains Mono",
					"{label}"
				}
			}
		}
	)
}

#[component]
fn settings_dropup_button(item: SettingsDropUpItem) -> Element {
	let theme = THEME_STORE().current_theme.colors;
	let mut hovered = use_signal(|| false); // required in future

	let animation = use_animation(move |conf| {
		conf.auto_start(false);

		AnimColor::new(
			&THEME_STORE().current_theme.colors.base,
			&THEME_STORE().current_theme.colors.surface1,
		)
		.time(150)
	});

	let bg_color = &*animation.get().read_unchecked();

	let label = item.name.clone();

	rsx!(
		CursorArea {
			icon: CursorIcon::Pointer,
			rect {
				width: "fill",
				height: "auto",
				background: "{bg_color.read()}",
				corner_radius: "10",
				padding: "5 6",
				onclick: move |_| (item.on_click)(),
				onmouseenter: move |_| {
					hovered.set(true);
					animation.start();
				},
				onmouseleave: move |_| {
					hovered.set(false);
					animation.reverse();
				},
				label {
					color:"{ theme.text }",
					font_size: "16",
					font_family: "JetBrains Mono",
					"{label}"
				}
			}
		}
	)
}
