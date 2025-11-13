// Copyright (C) 2025  Suyog Tandel(RedddFoxxyy)
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
