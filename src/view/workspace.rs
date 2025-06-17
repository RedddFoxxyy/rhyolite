use crate::{APP_THEME, GLOBAL_APP_STATE, view::bottom_bar::bottom_floating_bar};
use freya::prelude::*;

#[component]
pub fn work_space() -> Element {
	rsx!(rect {
		width: "100%",
		height: "100%",
		// bottom_floating_bar {  }
	})
}
