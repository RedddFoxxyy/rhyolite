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
