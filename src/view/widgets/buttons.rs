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

#[derive(PartialEq, Clone)]
pub enum _AnimationEasing {
	Linear,
	EaseIn,
	EaseOut,
	EaseInOut,
	Custom(f32), // For custom curves if supported
}

#[derive(PartialEq, Clone)]
pub struct _TransitionConfig {
	pub from_color: String,
	pub to_color: String,
	pub duration: u64,
	// pub ease: AnimationEasing,
	// pub delay: u64,
}
impl Default for _TransitionConfig {
	fn default() -> Self {
		Self {
			from_color: "transparent".to_string(),
			to_color: "#surface1".to_string(),
			duration: 150,
			// ease: AnimationEasing::EaseOut,
			// delay: 0,
		}
	}
}

impl _TransitionConfig {
	pub fn _with_colors(mut self, from: String, to: String) -> Self {
		self.from_color = from;
		self.to_color = to;
		self
	}
}

#[derive(PartialEq, Clone, Props, Default)]
pub struct DropDownButtonProps {
	#[props(default)]
	pub label: String,
	#[props(default = EventHandler::new(|_| {}))]
	pub onclick: EventHandler<()>,
	#[props(default = EventHandler::new(|_| {}))]
	pub onmouseenter: EventHandler<()>,
	#[props(default = EventHandler::new(|_| {}))]
	pub onmouseleave: EventHandler<()>,
	#[props(default)]
	pub icon: Option<&'static str>,
}

#[component]
pub fn DropDownButton(props: DropDownButtonProps) -> Element {
	let theme = THEME_STORE().current_theme.colors;
	let mut hovered = use_signal(|| false);

	// &THEME_STORE().current_theme.colors.base,
	// &THEME_STORE().current_theme.colors.surface1,
	let animation = use_animation(move |_conf| {
		AnimColor::new(
			&THEME_STORE().current_theme.colors.base,
			&THEME_STORE().current_theme.colors.surface1,
		)
		.time(150)
	});

	let bg_color = &*animation.get().read_unchecked();

	rsx!(
		CursorArea {
			icon: CursorIcon::Pointer,
			rect {
				width: "fill",
				height: "auto",
				background: "{bg_color.read()}",
				corner_radius: "10",
				padding: "5 6 5 6",
				direction: "horizontal",
				spacing: "5",
				onclick: move |_| props.onclick.call(()),
				onmouseenter: move |_| {
					hovered.set(true);
					animation.start();
					props.onmouseenter.call(());
				},
				onmouseleave: move |_| {
					hovered.set(false);
					animation.reverse();
					props.onmouseleave.call(());
				},

				if let Some(icon) = props.icon {
					rect {
						width: "20",
						height: "20",
						padding: "2",
						svg {
							width: "100%",
							height: "100%",
							stroke: "{ theme.text }",
							svg_content: icon
						}
					}
				}

				label {
					color:"{ theme.text }",
					font_size: "14",
					font_family: "JetBrains Mono",
					"{props.label}"
				}
			}
		}
	)
}
