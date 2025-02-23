use std::sync::{RwLockReadGuard, RwLockWriteGuard};

use crate::{app_state::TabManager, AppStateInner};

impl AppStateInner {
    /// Gets a read lock to the Tab Switcher in the app state, and returns it.
    /// If it fails to get a write lock, it emmits an error and returns
    /// a None Value.
    pub fn get_tab_manager(&self) -> Option<RwLockReadGuard<'_, TabManager>> {
        let tab_switcher = match self.tab_switcher.try_read() {
            Ok(lock) => lock,
            Err(err) => {
                log::error!("Cannot get a read lock on tab_switcher: {:?}", err);
                return None;
            }
        };
        Some(tab_switcher)
    }

    /// Gets a write lock to the Tab Switcher in the app state, and returns it.
    /// If it fails to get a write lock, it emmits an error and returns
    /// a None Value.
    pub fn get_tab_manager_mut(&self) -> Option<RwLockWriteGuard<'_, TabManager>> {
        let tab_switcher = match self.tab_switcher.try_write() {
            Ok(lock) => lock,
            Err(err) => {
                log::error!("Cannot get a write lock on tab_switcher: {:?}", err);
                return None;
            }
        };
        Some(tab_switcher)
    }
}
