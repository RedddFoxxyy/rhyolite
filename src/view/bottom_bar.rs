use crate::data::stores::{THEME_STORE, WORD_CHAR_COUNT};
use freya::prelude::*;

#[component]
pub fn bottom_floating_bar() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	// let bar_width = 260;
	// let bar_height = 30;

	rsx!(rect {
			position: "absolute",
			position_bottom: "10",
			position_right: "10",
			width: "fill",
			height: "30",
			// main_align: "end",
			cross_align: "end",
			rect {
			width: "auto",
			height: "fill",
			background: theme.base,
			shadow: "4 4 8 1 rgb(0, 0, 0, 10)",
			corner_radius: "100",
			padding: "1",
			layer: "overlay",
			rect {
				height: "fill",
				width: "auto",
				direction: "horizontal",
				main_align: "space-between",
				cross_align: "center",
				padding: "2 10",
				word_count {},
				char_count {}
			}
		}
	})
}

fn word_count() -> Element {
	let theme = THEME_STORE().current_theme.colors;
	rsx!(rect {
		width: "auto",
		main_align: "center",
		label {
			color: theme.text,
			font_size: "15",
			font_family: "JetBrains Mono",
			"{ WORD_CHAR_COUNT().0 } Words "
		}
	})
}

fn char_count() -> Element {
	let theme = THEME_STORE().current_theme.colors;
	rsx!(rect {
		width: "auto",
		main_align: "center",

		label {
			color: theme.text,
			font_size: "15",
			font_family: "JetBrains Mono",
			" { WORD_CHAR_COUNT().1 } Characters"
		}
	})
}
