#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod data;
mod view;

use data::{io_utils::env_logger_init, stores::ui_store::JET_BRAINS_MONO};
use freya::prelude::*;
use view::composite::app;

const APP_ICON: &[u8] = include_bytes!("./static/icon.png");

fn main() {
	env_logger_init();

	log::debug!("Rhyolite App started, initialising GUI.");
	launch_cfg(
		LaunchConfig::new()
			.with_font("JetBrains Mono", JET_BRAINS_MONO)
			.with_default_font("JetBrains Mono")
			.with_window(
				WindowConfig::new(app)
					.with_size(1284.0, 724.0)
					.with_title("Rhyolite")
					.with_min_size(400.0, 300.0)
					.with_decorations(false)
					.with_transparency(true)
					.with_background("transparent")
					.with_icon(LaunchConfig::load_icon(APP_ICON)),
			),
	);
}
