use crate::data::ui::THEME_STORE;
use freya::prelude::*;

#[component]
pub fn menu(children: Element) -> Element {
	let theme = THEME_STORE().current_theme.colors;

	// height: "auto",
	// position_bottom: "150",
	rsx!(
		rect {
			position: "global",
			width: "205",
			height: "132",
			position_bottom: "10",
			position_left: "55",
			padding: "6 4",
			background: "{theme.base}",
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
	// let themes_list = &THEME_STORE().store;
	rsx!(
		rect {
			position: "global",
			width: "220",
			height: "320",
			position_bottom: "10",
			position_left: "265",
			padding: "8 13 8 10",
			background: "{theme.base}",
			corner_radius: "12",
			ScrollView {
				width: "fill",
				direction: "vertical",
				spacing: "6",
				{children}
			}
		}
	)
}

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
	pub label: String,
	pub on_click: fn(&String),
	pub icon: Option<&'static str>,
}

#[component]
pub fn button(props: ButtonProps) -> Element {
	let theme = THEME_STORE().current_theme.colors;
	let mut hovered = use_signal(|| false); // required in future

	let animation = use_animation(move |conf| {
		conf.auto_start(false);

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
				padding: "5 6",
				direction: "horizontal",
				spacing: "5",
				// main_align: "space-between",
				onclick: move |_| (props.on_click)(&props.label),
				onmouseenter: move |_| {
					hovered.set(true);
					animation.start();
				},
				onmouseleave: move |_| {
					hovered.set(false);
					animation.reverse();
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
