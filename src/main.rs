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

/*
-------------------------------------------------------------------------
File Index
-------------------------------------------------------------------------
- Module Declarations
- Imports
- Constants and Static Variables
- Main Function
-------------------------------------------------------------------------
*/

#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

//-------------------------------------------------------------------------
// - Module Declarations
//-------------------------------------------------------------------------
mod data;
mod view;

//-------------------------------------------------------------------------
// - Imports
//-------------------------------------------------------------------------
use data::{io_utils::logger_init, stores::JET_BRAINS_MONO};
use freya::prelude::*;
use view::app_view::app;

//-------------------------------------------------------------------------
// - Constants and Static Variables
//-------------------------------------------------------------------------
const APP_ICON: &[u8] = include_bytes!("./static/icon.png");

//-------------------------------------------------------------------------
// - Main Function
//-------------------------------------------------------------------------
fn main() {
	logger_init();

	log::info!("Rhyolite App started, initialising GUI.");

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
