//! ## Module to store Helper Functions(Functions that encloses code that is to be reused
//! ## a lot of times in the code).
//!
//! As of now, this module contains functions that return variables stored in the
//! App State Struct, for example the tab_switcher which is locked behind an RWLock.
//! Functions like `get_tab_switcher()` or `get_tab_switcher_mut()` return a rwlockGuard
//! to the tab_switcher and also handle the errors if it cannot acquire a read/write
//! lock on it.
//!
//! These functions allow you to easily access the app_state variables
//! without having to deal with handling panics/errors and they do not cause any
//! deadlock if they fail!
//!
//! > NOTE: The module is unfinished as of now and more helper functions can be added, or the
//! > existing ones will be modified as required.

#![allow(dead_code)]

// use std::sync::{MutexGuard, RwLockReadGuard, RwLockWriteGuard};
use crate::{
    AppStateInner,
    // app_state::{CommandRegistry, FileManager, TabManager},
};
// use tokio::sync::{MutexGuard, RwLockReadGuard, RwLockWriteGuard};

impl AppStateInner {}
