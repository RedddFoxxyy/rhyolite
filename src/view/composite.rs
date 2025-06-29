use freya::prelude::*;
use crate::data::io_utils::initialise_app;
use crate::data::ui::THEME_STORE;
use crate::view::{docview::work_space, sidebar::side_bar, top_bar::top_nav_bar};

// The initial View for the app, all the app components are a part of this.
pub fn app() -> Element {
	let theme = THEME_STORE().current_theme.colors;

	use_hook(move || {
		initialise_app();
	});

	rsx!(rect {
		width: "fill",
		height: "fill",
		background: theme.crust,
		direction: "vertical",

		// Tabs Navigation Bar
		top_nav_bar {}

		// Main Workspace
		rect {
			width: "100%",
			height: "fill",
			direction: "horizontal",
			side_bar{},
			work_space{}
		}
	})
}
