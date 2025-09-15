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

use freya::prelude::*;

use crate::data::stores::THEME_STORE;

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
