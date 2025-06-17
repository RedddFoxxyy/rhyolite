use freya::prelude::*;

use crate::APP_THEME;
use crate::view::sidebar;
use crate::view::{sidebar::side_bar, top_bar::top_nav_bar, workspace::work_space};

// The initial View for the app, all the app components are a part of this.
pub fn app() -> Element {
	let background_color = APP_THEME.read().colors.crust.clone();

	rsx!(rect {
		width: "fill",
		height: "fill",
		background: background_color,
		direction: "vertical",
		top_nav_bar {}
		rect {
			width: "100%",
			height: "fill",
			direction: "horizontal",
			side_bar{},
			work_space{}
		}
	})
}
