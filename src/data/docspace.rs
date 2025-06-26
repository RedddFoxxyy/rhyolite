//! All global signals go here that store the state of the WorkSpace.
//!
//! I made these global signals because it was the easy way to share a component state between
//! different components. While this might not be the best way to do it, it works.

use freya::prelude::{GlobalSignal, Readable, Signal};
// use indexmap::IndexMap;

use crate::data::types::MarkdownFileData;

// Document Counts Store:
pub static WORD_CHAR_COUNT: GlobalSignal<(u32, u64)> = Signal::global(|| (0, 0));

// Documents Store:
pub static DOCUMENT_DATA: GlobalSignal<Vec<MarkdownFileData>> = Signal::global(Vec::new);

pub static DOCUMENT_TITLE: GlobalSignal<String> = Signal::global(String::new);
