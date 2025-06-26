#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

mod data;
mod utils;
mod view;

use data::ui::JET_BRAINS_MONO;
use freya::prelude::*;
use view::composite::app;

use crate::data::tabs::initialise_app;

const APP_ICON: &[u8] = include_bytes!("./static/icon.png");

fn main() {
	launch_cfg(
		app,
		LaunchConfig::<()>::new()
			.with_title("Rhyolite")
			.with_size(1280.0, 720.0)
			.with_font("JetBrains Mono", JET_BRAINS_MONO)
			.with_default_font("JetBrains Mono")
			.with_min_size(400.0, 300.0)
			// .with_decorations(false)
			.with_icon(LaunchConfig::load_icon(APP_ICON)),
	);
}
