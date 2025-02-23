#![allow(dead_code)]

use std::sync::{MutexGuard, RwLockReadGuard, RwLockWriteGuard};

use crate::{
    app_state::{CommandRegistry, FileManager, TabManager},
    AppStateInner,
};

impl AppStateInner {
    /// Gets a read lock to the Tab Switcher in the app state, and returns it.
    /// If it fails to get a write lock, it emmits an error and returns
    /// a None Value.
    pub fn get_tab_switcher(&self) -> Option<RwLockReadGuard<'_, TabManager>> {
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
    pub fn get_tab_switcher_mut(&self) -> Option<RwLockWriteGuard<'_, TabManager>> {
        let tab_switcher = match self.tab_switcher.try_write() {
            Ok(lock) => lock,
            Err(err) => {
                log::error!("Cannot get a write lock on tab_switcher: {:?}", err);
                return None;
            }
        };
        Some(tab_switcher)
    }

    pub fn get_command_registry(&self) -> Option<MutexGuard<'_, CommandRegistry>> {
        let command_registry = match self.command_registry.try_lock() {
            Ok(lock) => lock,
            Err(err) => {
                log::error!("Cannot get a mutex lock on command_registry: {:?}", err);
                return None;
            }
        };
        Some(command_registry)
    }

    pub fn get_workspace(&self) -> Option<RwLockReadGuard<'_, FileManager>> {
        let workspace = match self.workspace.try_read() {
            Ok(lock) => lock,
            Err(err) => {
                log::error!("Cannot get a read lock on workspace: {:?}", err);
                return None;
            }
        };
        Some(workspace)
    }

    pub fn get_workspace_mut(&self) -> Option<RwLockWriteGuard<'_, FileManager>> {
        let workspace = match self.workspace.try_write() {
            Ok(lock) => lock,
            Err(err) => {
                log::error!("Cannot get a write lock on workspace: {:?}", err);
                return None;
            }
        };
        Some(workspace)
    }
}
