use crate::{data::ui::THEME_STORE, view::bottom_bar::bottom_floating_bar};
use freya::prelude::*;

#[component]
pub fn work_space() -> Element {
	rsx!(rect {
		// Take the entire window width and height
		width: "fill",
		height: "fill",
		document_area{},
		bottom_floating_bar {  }
	})
}

fn document_area() -> Element {
	rsx!(rect {
		width: "fill",
		height: "fill",
		direction: "vertical",
		document_title_box{}
	})
}

fn document_title_box() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	rsx!(rect{
		width: "fill",
		height: "18%",
		min_height: "100",
		max_height: "120",
		main_align: "center",
		cross_align: "center",
		padding: "7",
		margin: "16 0 0 0",
		rect {
			width: "50%",
			min_width: "300",
			height: "fill",
			shadow: "5 8 8 2 rgb(0, 0, 0, 10)",
			background: "{theme.base}",
			corner_radius: "12",
			main_align: "center",
			padding: "4 12",
			label {
				color: "{theme.text}",
				font_size: "46",
				font_family: "JetBrains Mono",
				"Untitled"
			}
		}
	})
}
