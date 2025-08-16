use freya::prelude::*;

use crate::data::stores::ui_store::THEME_STORE;

#[component]
pub fn palette_box(children: Element) -> Element {
	let theme = THEME_STORE().current_theme.colors;
	let mut _focus = use_focus();
	rsx!(rect {
		width: "50%",
		height: "40%",
		min_width: "200",
		min_height: "100",
		max_height: "400",
		background: "{theme.crust}",
		shadow: "0 0 20 2 rgb(0, 0, 0, 102)",
		padding: "12",
		corner_radius: "8",
		corner_smoothing: "100%",
		onclick: move |e| {
			e.stop_propagation();
		} ,
		paragraph {
			text {
				color: "{theme.text}",
				font_size: "28",
				font_family: "JetBrains Mono",
				"To Be Implemented."
			}
		}
		{children}
	})
}
