//! All global signals go here that store the state of the Tabs.
//!
//! I made these global signals cause it was the easy way to share a component state between
//! different components. While this might not be the best way to do it, it works.

use crate::data::types::Tab;
use freya::prelude::{GlobalSignal, Readable, Signal};
use indexmap::IndexMap;

// Tabs Store:
pub static TABS: GlobalSignal<IndexMap<String, Tab>> = Signal::global(IndexMap::new);
pub static CURRENT_TAB: GlobalSignal<String> = Signal::global(String::new);
