use crate::{APP_THEME, GLOBAL_APP_STATE};
use freya::prelude::*;

#[component]
pub fn bottom_floating_bar() -> Element {
	let background_color = use_memo(move || APP_THEME.read().colors.base.clone());

	rsx!(rect {
		width: "120",
		height: "30",
		position: "absolute",
		position_bottom: "32",
		position_right: "122",
		background: "{background_color}",
		corner_radius: "50"
	})
}
