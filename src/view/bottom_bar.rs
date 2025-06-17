use crate::{APP_THEME, GLOBAL_APP_STATE};
use freya::prelude::*;

#[component]
pub fn bottom_floating_bar() -> Element {
	let background_color = use_memo(move || APP_THEME.read().colors.base.clone());

	let bar_width = 300;
	let bar_height = 35;

	rsx!(rect {
		width: "{ bar_width }",
		height: "{ bar_height }",
		position: "absolute",
		position_bottom: "10",
		position_right: "10",
		background: "{background_color}",
		shadow: "4 6 8 1 rgb(0, 0, 0, 20)",
		corner_radius: "100",
		padding: "1",
		rect {
			height: "fill",
			width: "fill",
			direction: "horizontal",
			main_align: "space-evenly",
			cross_align: "center",
			word_count {
			},
			char_count {
			}
		}
	})
}

fn word_count() -> Element {
	let text_color = use_memo(move || APP_THEME.read().colors.text.clone());
	rsx!(rect {
		width: "auto",
		main_align: "center",
		label {
			color: "{text_color}",
			font_size: "18",
			"0 Words"
		}
	})
}

fn char_count() -> Element {
	let text_color = use_memo(move || APP_THEME.read().colors.text.clone());
	rsx!(rect {
		width: "auto",
		main_align: "center",

		label {
			color: "{text_color}",
			font_size: "18",
			"0 Characters"
		}
	})
}
