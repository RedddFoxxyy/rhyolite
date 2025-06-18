//! All global signals go here that store the state of the UI.
//!
//! I made these global signals cause it was the easy way to share a component state between
//! different components. While this might not be the best way to do it, it works.

use freya::prelude::{GlobalSignal, Signal};

// Sidebar Stores:
pub static SHOW_SETTINGS_DROPUP: GlobalSignal<bool> = Signal::global(|| false);
pub static SHOW_COMMAND_PALETTE: GlobalSignal<bool> = Signal::global(|| false);
pub static SHOW_RECENT_FILES: GlobalSignal<bool> = Signal::global(|| false);
