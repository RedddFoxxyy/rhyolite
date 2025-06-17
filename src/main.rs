#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

mod app_state;
mod utils;
mod view;

use crate::utils::themes::Theme as UITheme;
use app_state::data_type::AppState;
use freya::prelude::*;
use view::composite::app;

const APP_ICON: &[u8] = include_bytes!("./static/icon.png");

// Stores all the data required to run the app.
pub static GLOBAL_APP_STATE: GlobalSignal<AppState> = Signal::global(AppState::default);

// Stores the current App Theme
pub static APP_THEME: GlobalSignal<UITheme> = Signal::global(UITheme::default);

fn main() {
	launch_cfg(
		app,
		LaunchConfig::<()>::new()
			.with_title("Rhyolite")
			.with_size(1280.0, 720.0)
			.with_min_size(200.0, 200.0)
			// .with_decorations(false)
			.with_icon(LaunchConfig::load_icon(APP_ICON)),
	)
}
