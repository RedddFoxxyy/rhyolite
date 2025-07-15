//! All global signals go here that store the state of the WorkSpace.
//!
//! I made these global signals because it was the easy way to share a component state between
//! different components. While this might not be the best way to do it, it works.

use freya::prelude::{GlobalSignal, Readable, Signal};

use crate::data::types::{FileInfo, MarkdownFile, UserData};
// TODO: Should each variable be stored seperate in a global signal or all should be grouped together in a struct and the struct be saved in a global sinal.

// Document Counts Store:
pub static WORD_CHAR_COUNT: GlobalSignal<(u32, u64)> = Signal::global(|| (0, 0));

// Documents Store:
pub static FILES_BUFFER: GlobalSignal<Vec<MarkdownFile>> = Signal::global(Vec::new);
pub static ACTIVE_DOCUMENT_TITLE: GlobalSignal<String> = Signal::global(String::new);

// IO Store:
pub static RECENT_FILES: GlobalSignal<Vec<FileInfo>> = Signal::global(Vec::new);
pub static USER_DATA: GlobalSignal<UserData> = Signal::global(UserData::default);
