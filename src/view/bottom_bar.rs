use crate::data::stores::{docspace::WORD_CHAR_COUNT, ui::THEME_STORE};
use freya::prelude::*;

#[component]
pub fn bottom_floating_bar() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	let bar_width = 250;
	let bar_height = 30;

	rsx!(rect {
		width: "{ bar_width }",
		height: "{ bar_height }",
		position: "absolute",
		position_bottom: "10",
		position_right: "10",
		background: theme.base,
		shadow: "4 4 8 1 rgb(0, 0, 0, 10)",
		corner_radius: "100",
		padding: "1",
		rect {
			height: "fill",
			width: "fill",
			direction: "horizontal",
			main_align: "space-evenly",
			cross_align: "center",
			word_count {},
			char_count {}
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
			"{ WORD_CHAR_COUNT().0 } Words"
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
			"{ WORD_CHAR_COUNT().1 } Characters"
		}
	})
}
