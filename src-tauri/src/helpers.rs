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

use std::sync::{MutexGuard, RwLockReadGuard, RwLockWriteGuard};

use crate::{
    app_state::{CommandRegistry, FileManager, TabManager},
    AppStateInner,
};

impl AppStateInner {
    /// ## Gets a read lock to the Tab Switcher in the app state, and returns it.
    /// If it fails to get a read lock, it emmits an error and returns
    /// a None Value.
    ///
    /// ___Example___:
    /// ```
    /// let maybe_tab_switcher = state.get_tab_switcher();
    /// if maybe_tab_switcher.is_none() {
    ///     error!("Failed to run the parent function!!")
    ///     return;
    /// }
    /// let tab_switcher = maybe_tab_switcher.unwrap();
    /// ```
    /// > NOTE: The above exaple uses an if guard clause to handle None
    /// > type vaule for the maybe_tab_switcher. However, you can use
    /// > alternatives like `if let Some(tab_switcher) = maybe_tab_switcher {}`
    /// > however using `if let` pattern matching, you have to keep in mind that
    /// > tab_switcher is not directly available but instead wrapped in a read guard!
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
    ///
    /// ___Example___:
    /// ```
    /// let maybe_tab_switcher = state.get_tab_switcher_mut();
    /// if maybe_tab_switcher.is_none() {
    ///     error!("Failed to run the parent function!!")
    ///     return;
    /// }
    /// let mut tab_switcher = maybe_tab_switcher.unwrap();
    /// ```
    /// The only difference between this and `get_tab_switcher` is that, this acquires
    /// a write lock and the previous one acquires a read lock, use either of them depending
    /// upon your requirements. Use this in case where you need a mutable access to the tab_switcher.
    ///
    /// > NOTE: The above exaple uses an if guard clause to handle None
    /// > type vaule for the maybe_tab_switcher. However, you can use
    /// > alternatives like `if let Some(tab_switcher) = maybe_tab_switcher {}`
    /// > however using `if let` pattern matching, you have to keep in mind that
    /// > tab_switcher is not directly available but instead wrapped in a write guard!
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

    /// ## Gets a read lock to the Workspace in the app state, and returns it.
    /// If it fails to get a read lock, it emmits an error and returns
    /// a None Value.
    ///
    /// ___Example___:
    /// ```
    /// let maybe_workspace = state.get_workspace();
    /// if maybe_workspace.is_none() {
    ///     error!("Failed to run the parent function!!")
    ///     return;
    /// }
    /// let workspace = maybe_workspace.unwrap();
    /// ```
    ///
    /// > NOTE: The above exaple uses an if guard clause to handle None
    /// > type vaule for the maybe_workspace. However, you can use
    /// > alternatives like `if let Some(workspace) = maybe_workspace {}`
    /// > however using `if let` pattern matching, you have to keep in mind that
    /// > workspace is not directly available but instead wrapped in a read guard!
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

    /// ## Gets a write lock to the Workspace in the app state, and returns it.
    /// If it fails to get a write lock, it emmits an error and returns
    /// a None Value.
    ///
    /// ___Example___:
    /// ```
    /// let maybe_workspace = state.get_workspace_mut();
    /// if maybe_workspace.is_none() {
    ///     error!("Failed to run the parent function!!")
    ///     return;
    /// }
    /// let mut workspace = maybe_workspace.unwrap();
    /// ```
    /// The only difference between this and `get_workspace` is that, this acquires
    /// a write lock and the previous one acquires a read lock, use either of them depending
    /// upon your requirements. Use this in case where you need a mutable access to the workspace.
    ///
    /// > NOTE: The above exaple uses an if guard clause to handle None
    /// > type vaule for the maybe_workspace. However, you can use
    /// > alternatives like `if let Some(workspace) = maybe_workspace {}`
    /// > however using `if let` pattern matching, you have to keep in mind that
    /// > workspace is not directly available but instead wrapped in a read guard!
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
