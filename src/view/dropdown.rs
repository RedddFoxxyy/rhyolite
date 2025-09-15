use crate::data::stores::THEME_STORE;
use freya::prelude::*;

#[component]
pub fn menu(children: Element) -> Element {
	let theme = THEME_STORE().current_theme.colors;

	rsx!(
		rect {
			position: "global",
			width: "205",
			height: "132",
			position_bottom: "10",
			position_left: "55",
			padding: "6 4",
			background: "{theme.base}",
			layer: "overlay",
			corner_radius: "12",
			rect {
				width: "fill",
				direction: "vertical",
				spacing: "6",

				{children}
			}
		},
	)
}

#[component]
pub fn submenu(children: Element) -> Element {
	let theme = THEME_STORE().current_theme.colors;

	let scrollbar_theme = theme_with!(ScrollBarTheme {
		background: cow_borrowed!("transparent"), //
		thumb_background: Cow::from(theme.surface0.clone()),
		hover_thumb_background: Cow::from(theme.surface1.clone()),
		active_thumb_background: Cow::from(theme.surface2.clone()),
	});

	rsx!(
		rect {
			position: "global",
			width: "220",
			height: "320",
			position_bottom: "10",
			position_left: "265",
			padding: "8 13 8 10",
			background: "{theme.base}",
			layer: "overlay",
			corner_radius: "12",
			ScrollView {
				width: "100%",
				direction: "vertical",
				spacing: "6",
				scrollbar_theme,
				{children}
			}
		}
	)
}
